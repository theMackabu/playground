use std::cell::RefCell;
use std::fmt::{self, Write};
use std::str;

use time::format_description::well_known::Rfc2822;
use time::{self, Duration, OffsetDateTime};

pub struct Now;

pub fn now() -> Now { Now }

struct LastRenderedNow {
    bytes: [u8; 128],
    amt: usize,
    next_update: OffsetDateTime,
}

thread_local! {
    static LAST: RefCell<LastRenderedNow> = RefCell::new(LastRenderedNow {
        amt: 0,
        bytes: [0; 128],
        next_update: OffsetDateTime::UNIX_EPOCH
    })
}

impl fmt::Display for Now {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        LAST.with(|cache| {
            let mut cache = cache.borrow_mut();
            let now = OffsetDateTime::now_utc();
            if now > cache.next_update {
                cache.update(now);
            }
            f.write_str(cache.buffer())
        })
    }
}

impl LastRenderedNow {
    fn buffer(&self) -> &str { str::from_utf8(&self.bytes[..self.amt]).unwrap() }

    fn update(&mut self, now: OffsetDateTime) {
        self.amt = 0;
        write!(LocalBuffer(self), "{}", now.format(&Rfc2822).unwrap()).unwrap();
        self.next_update = now + Duration::seconds(1);
        self.next_update = self.next_update.replace_nanosecond(0).unwrap();
    }
}

struct LocalBuffer<'a>(&'a mut LastRenderedNow);

impl<'a> fmt::Write for LocalBuffer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let start = self.0.amt;
        let end = start + s.len();
        self.0.bytes[start..end].copy_from_slice(s.as_bytes());
        self.0.amt += s.len();
        Ok(())
    }
}

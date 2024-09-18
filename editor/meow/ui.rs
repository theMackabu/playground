#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum Align {
    Bottom,
    Top,
    Left,
    Right,
}

pub enum Restriction {
    Grow,
    Shrink,
}

pub struct LayoutItem<'a, R: DrawResult, I, O: OutputResult> {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    widget: &'a dyn Widget<R, I, O>,
}

pub struct Layout<'a, R: DrawResult, I, O: OutputResult> {
    items: Vec<LayoutItem<'a, R, I, O>>,
    space: (u32, u32, u32, u32),
}

impl<'a, R: DrawResult, I, O: OutputResult> Layout<'a, R, I, O> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            space: (0, 0, width, height),
            items: Vec::new(),
        }
    }

    pub fn interact(self, interactions: &I) -> O { self.items.iter().map(|x| x.widget.interact(interactions, x.x, x.y, x.width, x.height)).fold(O::empty(), O::combine) }

    pub fn draw(self) -> R {
        let mut result = R::empty(self.space.2 - self.space.0, self.space.3 - self.space.1);
        let (mut x, mut y, mut width, mut height) = (self.space.0, self.space.1, self.space.2 - self.space.0, self.space.3 - self.space.1);

        for item in self.items.iter().rev() {
            let (new_x, new_y, new_width, new_height) = (
                x.min(item.x),
                y.min(item.y),
                if item.x != x || width == 0 { item.width + width } else { item.width },
                if item.y != y || height == 0 { item.height + height } else { item.height },
            );

            if item.x > x || width == 0 {
                result = result.combine_horizontal(item.widget.draw(item.width, item.height), new_width, width, new_height);
            } else if item.x < x {
                result = item.widget.draw(item.width, item.height).combine_horizontal(result, new_width, item.width, new_height);
            } else if item.y > y || height == 0 {
                result = result.combine_vertical(item.widget.draw(item.width, item.height), new_width, item.height, new_height);
            } else if item.y < y {
                result = item.widget.draw(item.width, item.height).combine_vertical(result, new_width, height, new_height);
            }

            x = new_x;
            y = new_y;
            width = new_width;
            height = new_height;
        }

        result
    }

    pub fn add_item(mut self, item: &'a dyn Widget<R, I, O>, align: Align, restriction: Restriction) -> Self {
        let expand_horizontal = match align {
            Align::Top | Align::Bottom => true,
            Align::Left | Align::Right => false,
        };

        let width = self.space.2 - self.space.0;
        let height = self.space.3 - self.space.1;

        let (min_width, min_height) = item.minimum_size(width, height);
        let (max_width, max_height) = item.maximum_size(width, height);

        let (widget_width, widget_height) = if expand_horizontal {
            (
                width,
                match restriction {
                    Restriction::Grow => max_height.min(height),
                    Restriction::Shrink => min_height.min(height),
                },
            )
        } else {
            (
                match restriction {
                    Restriction::Grow => max_width.min(width),
                    Restriction::Shrink => min_width.min(width),
                },
                height,
            )
        };

        let (widget_x, widget_y) = match align {
            Align::Top | Align::Left => (self.space.0, self.space.1),
            Align::Bottom => (self.space.0, self.space.3 - widget_height),
            Align::Right => (self.space.2 - widget_width, self.space.1),
        };

        self.items.push(LayoutItem {
            x: widget_x,
            y: widget_y,
            width: widget_width,
            height: widget_height,
            widget: item,
        });

        match align {
            Align::Top => self.space.1 += widget_height,
            Align::Bottom => self.space.3 -= widget_height,
            Align::Left => self.space.0 += widget_width,
            Align::Right => self.space.2 -= widget_width,
        }

        self
    }
}

pub trait Widget<R: DrawResult, I, O: OutputResult>: Drawable<R> + Interactive<I, O> {
    fn minimum_size(&self, width: u32, height: u32) -> (u32, u32);
    fn maximum_size(&self, width: u32, height: u32) -> (u32, u32);
}

pub trait Drawable<R: DrawResult> {
    fn draw(&self, width: u32, height: u32) -> R;
}

pub trait Interactive<I, O: OutputResult> {
    fn interact(&self, input: &I, x: u32, y: u32, width: u32, height: u32) -> O;
}

pub trait DrawResult {
    fn empty(width: u32, height: u32) -> Self;
    fn combine_horizontal(self, other: Self, width: u32, split: u32, height: u32) -> Self;
    fn combine_vertical(self, other: Self, width: u32, split: u32, height: u32) -> Self;
}

pub trait OutputResult {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

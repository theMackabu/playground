import React, { useState, useEffect, FC, ReactElement } from 'react';

// Interface for component props
interface CounterProps {
  initialCount: number;
  label: string;
}

// Type for the state
type CounterState = {
  count: number;
  isEven: boolean;
};

// Functional component with TypeScript
const Counter: FC<CounterProps> = ({ initialCount, label }): ReactElement => {
  const [state, setState] = useState<CounterState>({
    count: initialCount,
    isEven: initialCount % 2 === 0,
  });

  useEffect(() => {
    document.title = `${label}: ${state.count}`;
  }, [state.count, label]);

  const handleIncrement = (): void => {
    setState((prevState) => ({
      count: prevState.count + 1,
      isEven: (prevState.count + 1) % 2 === 0,
    }));
  };

  return (
    <div className="counter">
      <h2>{label}</h2>
      <p>Count: {state.count}</p>
      <p>The count is {state.isEven ? 'even' : 'odd'}</p>
      <button onClick={handleIncrement}>Increment</button>
    </div>
  );
};

// Interface for list item
interface ListItem {
  id: number;
  text: string;
}

// Generic component
function GenericList<T extends ListItem>({ items }: { items: T[] }): ReactElement {
  return (
    <ul>
      {items.map((item) => (
        <li key={item.id}>{item.text}</li>
      ))}
    </ul>
  );
}

// Enum
enum Theme {
  Light = 'light',
  Dark = 'dark',
}

// Context
const ThemeContext = React.createContext<Theme>(Theme.Light);

// Main App component
const App: FC = (): ReactElement => {
  const listItems: ListItem[] = [
    { id: 1, text: 'TypeScript' },
    { id: 2, text: 'React' },
    { id: 3, text: 'TSX' },
  ];

  return (
    <ThemeContext.Provider value={Theme.Light}>
      <div>
        <h1>TypeScript React Example</h1>
        <Counter initialCount={0} label="My Counter" />
        <GenericList items={listItems} />
      </div>
    </ThemeContext.Provider>
  );
};

export default App;

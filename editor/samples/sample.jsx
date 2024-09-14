import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import Button from '@/ui';

// Functional component with hooks
const ExampleComponent = ({ initialCount }) => {
  const [count, setCount] = useState(initialCount);
  const [isEven, setIsEven] = useState(initialCount % 2 === 0);

  useEffect(() => {
    document.title = `Count: ${count}`;
    setIsEven(count % 2 === 0);
  }, [count]);

  const handleIncrement = () => {
    setCount(prevCount => prevCount + 1);
  };

  return (
    <div className="example-component">
      <h1>Example JSX Component</h1>
      <p>Count: {count}</p>
      <p>The count is {isEven ? 'even' : 'odd'}</p>
      <Button onClick={handleIncrement}>Increment</Button>
      {count > 5 && <p>Count is greater than 5!</p>}
      <ul>
        {['React', 'JSX', 'Hooks'].map((item, index) => (
          <li key={index}>{item}</li>
        ))}
      </ul>
    </div>
  );
};

ExampleComponent.propTypes = {
  initialCount: PropTypes.number.isRequired,
};

// Class component
class ClassComponent extends React.Component {
  constructor(props) {
    super(props);
    this.state = { message: 'Hello from class component' };
  }

  render() {
    return <h2>{this.state.message}</h2>;
  }
}

// Higher-Order Component
const withLogging = (WrappedComponent) => {
  return class extends React.Component {
    componentDidMount() {
      console.log('Component mounted');
    }

    render() {
      return <WrappedComponent {...this.props} />;
    }
  };
};

const LoggedClassComponent = withLogging(ClassComponent);

// Main App component
const App = () => {
  return (
    <div>
      <ExampleComponent initialCount={0} />
      <LoggedClassComponent />
    </div>
  );
};

export default App;

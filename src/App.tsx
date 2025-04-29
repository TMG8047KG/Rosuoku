import "./App.css";
import Board from "./Board";

function App() {
  return (
    <div className="container">
    <h1>Rosuoku</h1>
    <p>A Rust + React Sudoku Game</p>
    <Board />
  </div>
  );
}

export default App;

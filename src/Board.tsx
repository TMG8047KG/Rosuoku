import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import styles from "./Board.module.css";

interface Cell {
  value: number;
  isOriginal: boolean;
  isValid: boolean;
  isHighlighted: boolean;
  hasError: boolean;
}

interface SudokuGrid {
  cells: number[][];
  solution: number[][];
  difficulty: number;
}

function Board() {
  const [grid, setGrid] = useState<Cell[][]>([]);
  const [solution, setSolution] = useState<number[][]>([]);
  const [selectedCell, setSelectedCell] = useState<[number, number] | null>(null);
  const [difficulty, setDifficulty] = useState(40);
  const [showError, setShowError] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");
  const [mistakes, setMistakes] = useState(0);
  const [showSuccess, setShowSuccess] = useState(false);
  const [timer, setTimer] = useState(0);
  const [timerActive, setTimerActive] = useState(false);

  useEffect(() => {
    generateNewGame();
  }, []);

  useEffect(() => {
    let interval: number | null = null;
    
    if (timerActive) {
      interval = window.setInterval(() => {
        setTimer(prevTime => prevTime + 1);
      }, 1000);
    }
    
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [timerActive]);

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const generateNewGame = async () => {
    try {
      const result: SudokuGrid = await invoke("generate_sudoku", { difficulty });
      
      const newGrid: Cell[][] = [];
      for (let i = 0; i < 9; i++) {
        newGrid[i] = [];
        for (let j = 0; j < 9; j++) {
          newGrid[i][j] = {
            value: result.cells[i][j],
            isOriginal: result.cells[i][j] !== 0,
            isValid: true,
            isHighlighted: false,
            hasError: false
          };
        }
      }
      
      setGrid(newGrid);
      setSolution(result.solution);
      setSelectedCell(null);
      setShowError(false);
      setMistakes(0);
      setShowSuccess(false);
      setTimer(0);
      setTimerActive(true);
    } catch (error) {
      console.error("Error generating Sudoku:", error);
    }
  };

  const handleCellClick = (row: number, col: number) => {
    setSelectedCell([row, col]);
    
    const cellHasError = grid[row][col].hasError;
    setShowError(cellHasError);
    if (cellHasError) {
      if (!grid[row][col].isValid) {
        setErrorMessage("This move creates a conflict!");
      } else if (grid[row][col].isHighlighted) {
        setErrorMessage("This number doesn't match the solution!");
      }
    }
  };

  const checkCompletion = async () => {
    const currentValues = grid.map(row => row.map(cell => cell.value));
    const isGridFilled = !currentValues.flat().includes(0);
    
    const allCorrect = grid.every((row, rowIndex) => 
      row.every((cell, colIndex) => 
        cell.value === 0 || cell.value === solution[rowIndex][colIndex]
      )
    );
    
    if (isGridFilled && allCorrect) {
      setShowSuccess(true);
      setTimerActive(false);
    }
  };

  const handleKeyDown = async (e: React.KeyboardEvent) => {
    if (!selectedCell) return;
    
    const [row, col] = selectedCell;
    
    if (grid[row][col].isOriginal) {
      setShowError(true);
      setErrorMessage("Can't modify original cells!");
      return;
    }
    
    if (e.key === "Backspace" || e.key === "Delete" || e.key === "0") {
      const newGrid = [...grid];
      newGrid[row][col] = { 
        ...newGrid[row][col], 
        value: 0, 
        isValid: true, 
        isHighlighted: false,
        hasError: false 
      };
      setGrid(newGrid);
      setShowError(false);
    } else if (/^[1-9]$/.test(e.key)) {
      const value = parseInt(e.key);
      
      const isCorrect = value === solution[row][col];
      
      const newGrid = [...grid];
      
      if (!isCorrect) {
        newGrid[row][col] = { 
          ...newGrid[row][col], 
          value, 
          isValid: true,
          isHighlighted: true,
          hasError: true 
        };
        setShowError(true);
        setErrorMessage("This number doesn't match the solution!");
        setMistakes(prev => prev + 1);
      } else {
        newGrid[row][col] = { 
          ...newGrid[row][col], 
          value, 
          isValid: true, 
          isHighlighted: false,
          hasError: false
        };
        setShowError(false);
      }
      
      setGrid(newGrid);
      
      await checkCompletion();
    }
  };

  const handleDifficultyChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setDifficulty(parseInt(e.target.value));
  };

  const resetGame = () => {
    generateNewGame();
  };

  return (
    <div className={styles.sudokuContainer} tabIndex={0} onKeyDown={handleKeyDown}>
      <div className={styles.gameHeader}>
        <div className={styles.difficultySelector}>
          <label htmlFor="difficulty">Difficulty: </label>
          <select
            id="difficulty"
            value={difficulty}
            onChange={handleDifficultyChange}
          >
            <option value={20}>Easy</option>
            <option value={40}>Medium</option>
            <option value={60}>Hard</option>
          </select>
        </div>
        
        <div className={styles.gameStats}>
          <div className={styles.timer}>Time: {formatTime(timer)}</div>
          <div className={styles.mistakes}>Mistakes: {mistakes}</div>
        </div>
        
        <button className={styles.newGameBtn} onClick={resetGame}>New Game</button>
      </div>
      
      {showError && (
        <div className={styles.errorMessage}>
          {errorMessage}
        </div>
      )}
      
      <div className={styles.sudokuGrid}>
        {grid.map((row, rowIndex) => (
          <div key={rowIndex} className={styles.sudokuRow}>
            {row.map((cell, colIndex) => {
              const cellClassNames = [styles.sudokuCell];
              
              if (selectedCell && selectedCell[0] === rowIndex && selectedCell[1] === colIndex) {
                cellClassNames.push(styles.selected);
              }
              
              if (cell.isOriginal) {
                cellClassNames.push(styles.original);
              }
              
              if (!cell.isValid) {
                cellClassNames.push(styles.invalid);
              }
              
              if (cell.isHighlighted) {
                cellClassNames.push(styles.highlighted);
              }
              
              if (rowIndex % 3 === 2 && rowIndex < 8) {
                cellClassNames.push(styles.borderBottom);
              }
              
              if (colIndex % 3 === 2 && colIndex < 8) {
                cellClassNames.push(styles.borderRight);
              }
              
              return (
                <div
                  key={`${rowIndex}-${colIndex}`}
                  className={cellClassNames.join(' ')}
                  onClick={() => handleCellClick(rowIndex, colIndex)}
                >
                  {cell.value !== 0 ? cell.value : ''}
                </div>
              );
            })}
          </div>
        ))}
      </div>
      
      {showSuccess && (
        <div className={styles.successOverlay}>
          <div className={styles.successModal}>
            <h2>Congratulations!</h2>
            <p>You've solved the puzzle!</p>
            <p>Time: {formatTime(timer)}</p>
            <p>Mistakes: {mistakes}</p>
            <button onClick={resetGame}>Play Again</button>
          </div>
        </div>
      )}
      
      <div className={styles.instructions}>
        Click on a cell and type a number (1-9) to fill it in.
        Use Backspace or Delete to clear a cell.
      </div>
    </div>
  );
}
export default Board;
use rand::{seq::SliceRandom, rng};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SudokuGrid {
    cells: Vec<Vec<u8>>,
    solution: Vec<Vec<u8>>,
    difficulty: usize,
}

#[tauri::command]
pub fn generate_sudoku(difficulty: usize) -> SudokuGrid {
    let mut grid = SudokuGrid::new(difficulty);
    grid.generate();
    grid
}

impl SudokuGrid {
    pub fn new(difficulty: usize) -> Self {
        let cells = vec![vec![0; 9]; 9];
        let solution = vec![vec![0; 9]; 9];
        Self { cells, solution, difficulty }
    }

    // Generate a valid Sudoku puzzle with a unique solution
    pub fn generate(&mut self) {
        // First, generate a solved grid
        self.solve_empty();
        
        // Store the solution
        self.solution = self.cells.clone();
        
        // Remove cells while ensuring a unique solution
        self.remove_cells_unique();
    }

    // Solve an empty grid
    fn solve_empty(&mut self) {
        // Fill in the diagonal 3x3 boxes first (these can be filled independently)
        for i in 0..3 {
            self.fill_box(i * 3, i * 3);
        }
        
        // Solve the rest of the grid
        self.solve_grid(0, 0);
    }
    
    // Fill a 3x3 box with numbers 1-9
    fn fill_box(&mut self, row: usize, col: usize) {
        let mut numbers: Vec<u8> = (1..=9).collect();
        numbers.shuffle(&mut rng());
        
        let mut index = 0;
        for i in 0..3 {
            for j in 0..3 {
                self.cells[row + i][col + j] = numbers[index];
                index += 1;
            }
        }
    }
    
    // Check if it's safe to place a number
    fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        // Check row
        for x in 0..9 {
            if self.cells[row][x] == num {
                return false;
            }
        }
        
        // Check column
        for x in 0..9 {
            if self.cells[x][col] == num {
                return false;
            }
        }
        
        // Check 3x3 box
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        
        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }
        
        true
    }
    
    // Recursively solve the grid
    fn solve_grid(&mut self, row: usize, col: usize) -> bool {
        if row == 9 {
            return true; // We've filled the entire grid
        }
        
        // Calculate next position
        let (next_row, next_col) = if col == 8 {
            (row + 1, 0)
        } else {
            (row, col + 1)
        };
        
        // If the cell is already filled, move to the next cell
        if self.cells[row][col] != 0 {
            return self.solve_grid(next_row, next_col);
        }
        
        // Try digits 1-9
        let mut numbers: Vec<u8> = (1..=9).collect();
        numbers.shuffle(&mut rng());
        
        for &num in &numbers {
            if self.is_safe(row, col, num) {
                self.cells[row][col] = num;
                
                if self.solve_grid(next_row, next_col) {
                    return true;
                }
                
                self.cells[row][col] = 0; // Backtrack
            }
        }
        
        false
    }
    
    // Remove cells while ensuring the puzzle has a unique solution
    fn remove_cells_unique(&mut self) {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                positions.push((i, j));
            }
        }
        
        positions.shuffle(&mut rng());
        
        // Limit how many cells to try removing based on difficulty
        let max_cells_to_remove = match self.difficulty {
            d if d <= 20 => 30,  // Easy - around 51 cells remain
            d if d <= 40 => 45,  // Medium - around 36 cells remain
            _ => 55,             // Hard - around 26 cells remain
        };
        
        // Try to remove cells one by one, but keep the puzzle with a unique solution
        let mut cells_removed = 0;
        for (row, col) in positions {
            // Skip if we've already removed enough cells
            if cells_removed >= max_cells_to_remove {
                break;
            }
            
            let temp = self.cells[row][col];
            self.cells[row][col] = 0;
            
            // Count solutions
            let solutions_count = self.count_solutions();
            
            if solutions_count != 1 {
                // More than one solution means this cell is necessary for uniqueness
                self.cells[row][col] = temp;
            } else {
                cells_removed += 1;
            }
        }
    }
    
    // Count the number of solutions this puzzle has
    // Returns: 0 if no solution, 1 if unique solution, 2 if multiple solutions
    fn count_solutions(&self) -> usize {
        // Create a clone to work with
        let mut grid_clone = self.cells.clone();
        let mut solution_count = 0;
        
        // Find the first empty cell
        let mut found_empty = false;
        let mut row = 0;
        let mut col = 0;
        
        'outer: for r in 0..9 {
            for c in 0..9 {
                if grid_clone[r][c] == 0 {
                    row = r;
                    col = c;
                    found_empty = true;
                    break 'outer;
                }
            }
        }
        
        // If no empty cells found, we have a solution
        if !found_empty {
            return 1;
        }
        
        // Try different values for this cell
        for num in 1..=9 {
            if self.is_valid_placement(&grid_clone, row, col, num) {
                grid_clone[row][col] = num;
                
                // Recursively count solutions
                solution_count += self.count_solutions_recursive(&mut grid_clone);
                
                // No need to explore further if we found multiple solutions
                if solution_count > 1 {
                    return 2;
                }
                
                // Backtrack
                grid_clone[row][col] = 0;
            }
        }
        
        solution_count
    }
    
    // Recursive helper for counting solutions
    fn count_solutions_recursive(&self, grid: &mut Vec<Vec<u8>>) -> usize {
        // Find the first empty cell
        let mut found_empty = false;
        let mut row = 0;
        let mut col = 0;
        
        'outer: for r in 0..9 {
            for c in 0..9 {
                if grid[r][c] == 0 {
                    row = r;
                    col = c;
                    found_empty = true;
                    break 'outer;
                }
            }
        }
        
        // If no empty cells found, we have a solution
        if !found_empty {
            return 1;
        }
        
        let mut solution_count = 0;
        
        // Try different values for this cell
        for num in 1..=9 {
            if self.is_valid_placement(grid, row, col, num) {
                grid[row][col] = num;
                
                // Recursively count solutions
                solution_count += self.count_solutions_recursive(grid);
                
                // No need to explore further if we found multiple solutions
                if solution_count > 1 {
                    return 2;
                }
                
                // Backtrack
                grid[row][col] = 0;
            }
        }
        
        solution_count
    }
    
    // Check if placing a number is valid
    fn is_valid_placement(&self, grid: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
        // Check row
        for x in 0..9 {
            if grid[row][x] == num {
                return false;
            }
        }
        
        // Check column
        for x in 0..9 {
            if grid[x][col] == num {
                return false;
            }
        }
        
        // Check 3x3 box
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        
        for i in 0..3 {
            for j in 0..3 {
                if grid[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }
        
        true
    }
}

#[tauri::command]
pub fn check_solution(grid: Vec<Vec<u8>>, solution: Vec<Vec<u8>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] != 0 && grid[i][j] != solution[i][j] {
                return false;
            }
        }
    }
    true
}

#[tauri::command]
pub fn is_valid_move(grid: Vec<Vec<u8>>, row: usize, col: usize, value: u8) -> bool {
    if value == 0 {
        return true; // Can always clear a cell
    }
    
    // Check row
    for x in 0..9 {
        if x != col && grid[row][x] == value {
            return false;
        }
    }
    
    // Check column
    for x in 0..9 {
        if x != row && grid[x][col] == value {
            return false;
        }
    }
    
    // Check 3x3 box
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    
    for i in 0..3 {
        for j in 0..3 {
            let r = i + start_row;
            let c = j + start_col;
            if r != row && c != col && grid[r][c] == value {
                return false;
            }
        }
    }
    
    true
}

#[tauri::command]
pub fn is_complete(grid: Vec<Vec<u8>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == 0 {
                return false;
            }
        }
    }
    
    true
}
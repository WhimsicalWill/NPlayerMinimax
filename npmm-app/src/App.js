import React, { useState, useEffect } from "react";
import "./App.css";
import init, {
    initialize_game,
    get_board,
    get_to_move,
    get_num_moves,
    game_status,
    make_human_move,
    make_ai_move,
} from "./pkg/npmm.js";

const DEFAULT_ROWS = 6;
const DEFAULT_COLS = 7;
const DEFAULT_PLAYERS = 2;
const DEFAULT_N_IN_A_ROW = 4;
const DEFAULT_SEARCH_DEPTH = 7;

function App() {
    const [board, setBoard] = useState([]);
    const [toMove, setToMove] = useState(0);
    const [moveNum, setMoveNum] = useState(0);
    const [status, setStatus] = useState(-2); // -2 means game is ongoing

    useEffect(() => {
        init().then(() => {
            handleReset();
        });
    }, []);

    const handleReset = () => {
        initialize_game(
            DEFAULT_ROWS,
            DEFAULT_COLS,
            DEFAULT_PLAYERS,
            DEFAULT_N_IN_A_ROW,
            DEFAULT_SEARCH_DEPTH
        );
        updateGameState();
    };

    const handleClick = (col) => {
        make_human_move(col);
        updateGameState();
    };

    // A function to update the React state after each move
    const updateGameState = () => {
        setBoard(get_board());
        setToMove(get_to_move());
        setMoveNum(get_num_moves());
        setStatus(game_status());
    };

    // Trigger the AI move if appropriate
    useEffect(() => {
        if (toMove === 1 && status === -2) {
            make_ai_move();
            updateGameState();
        }
    }, [toMove, status]);

    let statusText;
    switch (status) {
        case -2:
            statusText = "Game Ongoing";
            break;
        case -1:
            statusText = "It's a Tie!";
            break;
        default:
            statusText = `Player ${status + 1} Wins!`;
            break;
    }

    return (
        <div className="App">
            <div className="game-container">
                <div className="board">
                    {board.map((row, rowIndex) => (
                        <div className="row" key={rowIndex}>
                            {row.map((cell, cellIndex) => (
                                <button
                                    className="cell"
                                    key={cellIndex}
                                    onClick={() =>
                                        status === -2 && handleClick(cellIndex)
                                    }
                                    disabled={status !== -2}
                                >
                                    {cell}
                                </button>
                            ))}
                        </div>
                    ))}
                </div>
                <div className="info-panel">
                    <p>Next Move: {toMove === 0 ? "You" : "AI"}</p>
                    <p>Total Moves: {moveNum}</p>
                    <p>{statusText}</p>
                    {status !== -2 && (
                        <button className="reset-button" onClick={handleReset}>
                            Reset Game
                        </button>
                    )}
                </div>
            </div>
        </div>
    );
}

export default App;

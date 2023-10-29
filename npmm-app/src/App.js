import React, { useState, useCallback, useEffect, useRef } from "react";
import "./App.css";
import init, { create_game_controller } from "./pkg/npmm.js";

function App() {
    const gameControllerRef = useRef(null);
    const [board, setBoard] = useState([]);
    const [toMove, setToMove] = useState(0);
    const [moveNum, setMoveNum] = useState(0);
    const [status, setStatus] = useState(-2); // -2 means game is ongoing

    // A function to update the React state after each move
    const updateGameState = useCallback(() => {
        setBoard(gameControllerRef.current.get_board());
        setToMove(gameControllerRef.current.get_to_move());
        setMoveNum(gameControllerRef.current.get_move_num());
        setStatus(gameControllerRef.current.get_game_status());
    }, []);

    const handleReset = useCallback(() => {
        gameControllerRef.current = create_game_controller();
        updateGameState();
    }, [updateGameState]);

    useEffect(() => {
        init().then(handleReset);
    }, [handleReset]);

    const handleClick = (col) => {
        gameControllerRef.current.make_human_move(col);
        updateGameState();
    };

    useEffect(() => {
        if (toMove === 1 && status === -2) {
            setTimeout(() => {
                gameControllerRef.current.make_ai_move();
                updateGameState();
            }, 25);
        }
    }, [toMove, status, updateGameState]);

    const getStatusText = (status) => {
        switch (status) {
            case -2:
                return "Game Ongoing";
            case -1:
                return "It's a Tie!";
            default:
                return `Player ${status + 1} Wins!`;
        }
    };

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
                                    onClick={() => handleClick(cellIndex)}
                                    disabled={status !== -2 || toMove !== 0}
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
                    <p>{getStatusText(status)}</p>
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

import React, { useState, useCallback, useEffect, useRef } from "react";
import GameDescriptionBox from "./GameDescriptionBox";
import "./App.css";

function App() {
    const gameControllerRef = useRef(null);
    const [board, setBoard] = useState([]);
    const [toMove, setToMove] = useState(0);
    const [moveNum, setMoveNum] = useState(0);
    const [status, setStatus] = useState(null);
    const [numPlayers, setNumPlayers] = useState(2);
    const [availableMoves, setAvailableMoves] = useState([]);
    const [wasmModule, setWasmModule] = useState(null);

    // A function to update the React state after each move
    const updateGameState = useCallback(() => {
        console.log(gameControllerRef.current);
        setBoard(gameControllerRef.current.get_board());
        setToMove(gameControllerRef.current.get_to_move());
        setMoveNum(gameControllerRef.current.get_move_num());
        setStatus(gameControllerRef.current.get_game_status());
        setAvailableMoves(gameControllerRef.current.get_valid_moves());
    }, []);

    const handleReset = useCallback(() => {
        if (wasmModule) {
            gameControllerRef.current = numPlayers; // create_game_controller(numPlayers);
            console.log(gameControllerRef.current);
            updateGameState();
        }
    }, [numPlayers, updateGameState, wasmModule]);

    const handlePlayerSelection = (players) => {
        setNumPlayers(players);
        handleReset();
    };

    // Reset the game when the wasmModule loads (after game is created)
    useEffect(() => {
        if (wasmModule) {
            handleReset();
        }
    }, [handleReset, wasmModule]);

    const handleClick = (row, col) => {
        console.log(row, col);
        gameControllerRef.current.make_human_move(row, col);
        updateGameState();
    };

    useEffect(() => {
        if (
            wasmModule &&
            toMove !== wasmModule.Player.Player0 &&
            status === wasmModule.GameStatus.Ongoing
        ) {
            setTimeout(() => {
                gameControllerRef.current.make_ai_move();
                updateGameState();
            }, 25);
        }
    }, [toMove, status, updateGameState, wasmModule]);

    const getStatusText = (status) => {
        if (!wasmModule) return "";
        switch (status) {
            case wasmModule.GameStatus.Ongoing:
                return "Game Ongoing";
            case wasmModule.GameStatus.Tie:
                return "It's a Tie!";
            default:
                return `Player ${status} Wins!`;
        }
    };

    return (
        <div className="App">
            <div className="player-selection">
                {[2, 3, 4].map((num) => (
                    <button
                        key={num}
                        onClick={() => handlePlayerSelection(num)}
                        className={numPlayers === num ? "selected" : ""}
                    >
                        {num} Players
                    </button>
                ))}
            </div>
            <GameDescriptionBox
                wasmModule={wasmModule}
                setWasmModule={setWasmModule}
            />
            <div className="game-container">
                <div className="board">
                    {board.map((row, rowIndex) => (
                        <div className="row" key={rowIndex}>
                            {row.map((cell, cellIndex) => {
                                // Check if this cell is a valid move
                                const isValidMove = availableMoves.some(
                                    (move) =>
                                        move[0] === rowIndex &&
                                        move[1] === cellIndex
                                );

                                // Conditional className based on whether the cell is a valid move
                                const cellClass = isValidMove
                                    ? "cell valid-move"
                                    : "cell";

                                return (
                                    <button
                                        className={cellClass}
                                        key={cellIndex}
                                        onClick={() =>
                                            handleClick(rowIndex, cellIndex)
                                        }
                                        disabled={
                                            wasmModule &&
                                            (status !==
                                                wasmModule.GameStatus.Ongoing ||
                                                toMove !==
                                                    wasmModule.Player.Player0 ||
                                                !isValidMove)
                                        }
                                    >
                                        {cell}
                                    </button>
                                );
                            })}
                        </div>
                    ))}
                </div>
                {wasmModule && (
                    <div className="info-panel">
                        <p>
                            Next Move:{" "}
                            {toMove === wasmModule.Player.Player0
                                ? "You"
                                : "AI"}
                        </p>
                        <p>Total Moves: {moveNum}</p>
                        <p>{getStatusText(status)}</p>
                        {status !== wasmModule.GameStatus.Ongoing && (
                            <button
                                className="reset-button"
                                onClick={handleReset}
                            >
                                Reset Game
                            </button>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

export default App;

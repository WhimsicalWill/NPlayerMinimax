import React, { useState, useCallback, useEffect, useRef } from "react";
import "./App.css";
import init, {
    create_game_controller,
    GameStatus,
    Player,
} from "./pkg/npmm.js";

function App() {
    const gameControllerRef = useRef(null);
    const [board, setBoard] = useState([]);
    const [toMove, setToMove] = useState(0);
    const [moveNum, setMoveNum] = useState(0);
    const [status, setStatus] = useState(GameStatus.Ongoing);
    const [numPlayers, setNumPlayers] = useState(2);
    const [availableMoves, setAvailableMoves] = useState([]);

    // A function to update the React state after each move
    const updateGameState = useCallback(() => {
        setBoard(gameControllerRef.current.get_board());
        setToMove(gameControllerRef.current.get_to_move());
        setMoveNum(gameControllerRef.current.get_move_num());
        setStatus(gameControllerRef.current.get_game_status());
        setAvailableMoves(gameControllerRef.current.get_valid_moves());
    }, []);

    const handleReset = useCallback(() => {
        gameControllerRef.current = create_game_controller(numPlayers);
        updateGameState();
    }, [numPlayers, updateGameState]);

    const handlePlayerSelection = (players) => {
        setNumPlayers(players);
        handleReset();
    };

    useEffect(() => {
        init().then(handleReset);
    }, [handleReset]);

    const handleClick = (row, col) => {
        console.log(row, col);
        gameControllerRef.current.make_human_move(row, col);
        updateGameState();
    };

    useEffect(() => {
        if (toMove !== Player.Player0 && status === GameStatus.Ongoing) {
            setTimeout(() => {
                gameControllerRef.current.make_ai_move();
                updateGameState();
            }, 25);
        }
    }, [toMove, status, updateGameState]);

    const getStatusText = (status) => {
        switch (status) {
            case GameStatus.Ongoing:
                return "Game Ongoing";
            case GameStatus.Tie:
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
                                            status !== GameStatus.Ongoing ||
                                            toMove !== Player.Player0 ||
                                            !isValidMove
                                        }
                                    >
                                        {cell}
                                    </button>
                                );
                            })}
                        </div>
                    ))}
                </div>
                <div className="info-panel">
                    <p>Next Move: {toMove === Player.Player0 ? "You" : "AI"}</p>
                    <p>Total Moves: {moveNum}</p>
                    <p>{getStatusText(status)}</p>
                    {status !== GameStatus.Ongoing && (
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

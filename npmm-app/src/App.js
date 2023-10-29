import React, { useState, useEffect } from "react";
import "./App.css";
import init, {
    create_game_controller,
    make_human_move,
    make_ai_move,
} from "./pkg/npmm.js";

function App() {
    const [gameController, setGameController] = useState(null);

    useEffect(() => {
        init().then(handleReset);
    }, []);

    const handleReset = () => {
        setGameController(create_game_controller());
    };

    const handleClick = (col) => {
        make_human_move(gameController, col);
        setGameController({ ...gameController });
    };

    useEffect(() => {
        if (
            gameController?.get_to_move() === 1 &&
            gameController?.get_game_status() === -2
        ) {
            setTimeout(() => {
                make_ai_move(gameController);
                setGameController({ ...gameController });
            }, 25);
        }
    }, [gameController]);

    let statusText;
    const status = gameController?.get_game_status();
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
                {/* ... */}
                <div className="board">
                    {gameController?.get_board().map((row, rowIndex) => (
                        <div className="row" key={rowIndex}>
                            {row.map((cell, cellIndex) => (
                                <button
                                    className="cell"
                                    key={cellIndex}
                                    onClick={() => handleClick(cellIndex)}
                                    disabled={
                                        status !== -2 ||
                                        gameController?.get_to_move() !== 0
                                    }
                                >
                                    {cell}
                                </button>
                            ))}
                        </div>
                    ))}
                </div>
                <div className="info-panel">
                    <p>
                        Next Move:{" "}
                        {gameController?.get_to_move() === 0 ? "You" : "AI"}
                    </p>
                    <p>Total Moves: {gameController?.get_move_num()}</p>
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

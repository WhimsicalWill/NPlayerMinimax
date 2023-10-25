import React, { useState, useEffect } from "react";
import "./App.css";
import init, {
    initialize_game,
    get_board,
    get_to_move,
    get_num_moves,
    make_move,
} from "./pkg/npmm.js";

function App() {
    const [board, setBoard] = useState([]);
    const [toMove, setToMove] = useState(0);
    const [moveNum, setMoveNum] = useState(0);

    useEffect(() => {
        init().then(() => {
            initialize_game(6, 7, 2, 4);
            setBoard(get_board());
            setToMove(get_to_move());
            setMoveNum(get_num_moves());
        });
    }, []);

    const handleClick = (col) => {
        make_move(col);
        setBoard(get_board());
        setToMove(get_to_move());
        setMoveNum(get_num_moves());
    };

    return (
        <div className="App">
            <header className="App-header">
                <div>
                    {board.map((row, rowIndex) => (
                        <div key={rowIndex}>
                            {row.map((cell, cellIndex) => (
                                <button
                                    key={cellIndex}
                                    onClick={() => handleClick(cellIndex)}
                                >
                                    {cell === 0 ? "-" : cell === 1 ? "X" : "O"}
                                </button>
                            ))}
                        </div>
                    ))}
                </div>
                <p>Next Move: Player {toMove}</p>
                <p>Total Moves: {moveNum}</p>
            </header>
        </div>
    );
}

export default App;

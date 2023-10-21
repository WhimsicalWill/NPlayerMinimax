import React, { useState, useEffect, useRef } from "react";
import "./App.css";

function App() {
    const [board, setBoard] = useState([]);
    const [toMove, setToMove] = useState(0);
    const [moveNum, setMoveNum] = useState(0);
    const [isWasmReady, setWasmReady] = useState(false);

    const gameRef = useRef();
    const wasmRef = useRef(null); // To store the wasm module

    useEffect(() => {
        const initWasm = async () => {
            const wasmModule = await import("./wasm/npmm.js");
            wasmRef.current = wasmModule;
            setWasmReady(true);
        };

        initWasm();
    }, []);

    useEffect(() => {
        if (!isWasmReady) return;

        gameRef.current = new wasmRef.current.PushUpFourGame(6, 7, 2, 4);
        const initialGameState = gameRef.current.get_state();

        setBoard(initialGameState.board);
        setToMove(initialGameState.to_move);
        setMoveNum(initialGameState.move_num);
    }, [isWasmReady]);

    const handleClick = (col) => {
        const newState = gameRef.current.make_move(col);
        setBoard(newState.board);
        setToMove(newState.to_move);
        setMoveNum(newState.move_num);
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
                                    {cell}
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

import React, { useState } from "react";
import axios from "axios";
import JSZip from "jszip";
import init from "./npmm.js";

// Set the default base URL for axios
axios.defaults.baseURL = "http://127.0.0.1:8080";

function GameDescriptionBox({ wasmModule, setWasmModule }) {
    const [description, setDescription] = useState("");

    const handleDescriptionChange = (e) => {
        setDescription(e.target.value);
    };

    const fetchAndLoadWasm = async () => {
        try {
            console.log("Submitted description:", description);

            // Use axios to make an API call to your backend
            const response = await axios.post(
                "/generate-and-compile",
                {
                    prompt: description,
                },
                {
                    responseType: "arraybuffer",
                }
            );

            console.log("Fetching the WASM package...");
            // const zip = await JSZip.loadAsync(response.data);
            // console.log(zip);
            // const wasmFileContent = await zip
            //     .file("pkg\\npmm_bg.wasm")
            //     .async("arraybuffer");

            import("http://127.0.0.1:8080/pkg/npmm.js").then(
                ({ default: init, create_game_controller }) => {
                    init().then(() => {
                        const gameContr = create_game_controller(2);
                        console.log(gameContr.get_board());
                        console.log(gameContr.get_valid_moves());
                    });
                }
            );
        } catch (error) {
            console.error(
                "Error in fetching and initializing the WebAssembly module:",
                error
            );
        }
    };

    const handleDescriptionSubmit = async () => {
        fetchAndLoadWasm();
    };

    return (
        <div>
            <textarea
                className="game-description-box"
                placeholder="Describe the game you want to play..."
                value={description}
                onChange={handleDescriptionChange}
                rows={4}
                cols={50}
            />
            <button
                className="game-description-button"
                onClick={handleDescriptionSubmit}
            >
                Submit
            </button>
        </div>
    );
}

export default GameDescriptionBox;

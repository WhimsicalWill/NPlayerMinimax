import React, { useState } from "react";
import axios from "axios";
import JSZip from "jszip";

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
            const zip = await JSZip.loadAsync(response.data);
            const jsFileContent = await zip.file("pkg\\npmm.js");
            const jsFileBlob = new Blob([jsFileContent], {
                type: "application/javascript",
            });
            const jsFileUrl = URL.createObjectURL(jsFileBlob);

            import(jsFileUrl).then(async (module) => {
                // Use the module's exported functions here
                await module.default(); // This is the equivalent of calling 'init()' in your previous setup.
                setWasmModule(module);
            });
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

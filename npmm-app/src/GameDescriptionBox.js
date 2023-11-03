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
            const fileNames = [];
            zip.forEach((relativePath, file) => {
                fileNames.push(relativePath);
            });
            console.log("Files in ZIP:", fileNames);

            const jsFileBlob = await zip.file("pkg\\npmm.js").async("blob");
            const wasmFileBlob = await zip
                .file("pkg\\npmm_bg.wasm")
                .async("blob");

            const jsFileUrl = URL.createObjectURL(jsFileBlob);
            const wasmFileUrl = URL.createObjectURL(wasmFileBlob);

            const loadedWasmModule = await import(jsFileUrl);
            await loadedWasmModule.default(wasmFileUrl); // This is equivalent to `init()`
            setWasmModule(loadedWasmModule);
        } catch (error) {
            console.error("Error in fetching response:", error);
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

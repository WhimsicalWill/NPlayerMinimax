import React, { useState } from "react";

function GameDescriptionBox() {
    const [description, setDescription] = useState("");

    const handleDescriptionChange = (e) => {
        setDescription(e.target.value);
    };

    const handleDescriptionSubmit = () => {
        // Implement the API call logic here
        console.log("Submitted description:", description);
        // Example: Make an API call
        // axios.post('/api/game-description', { description })
        //     .then(response => console.log(response))
        //     .catch(error => console.error(error));
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

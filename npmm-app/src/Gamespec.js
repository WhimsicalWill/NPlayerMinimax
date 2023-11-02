import { OpenAIApi, Configuration } from "openai";
import {
    SYSTEM_PROMPT,
    EXAMPLE_SPEC,
    EXAMPLE_RESPONSE,
} from "./PromptExamples.js";

// Set up the OpenAI API configuration
const openAIConfig = new Configuration({
    apiKey: process.env.OPENAI_API_KEY,
});
const openai = new OpenAIApi(openAIConfig);

async function fetchOpenAIResponse(prompt) {
    const messages = [
        { role: "system", content: SYSTEM_PROMPT },
        { role: "user", content: EXAMPLE_SPEC },
        { role: "assistant", content: EXAMPLE_RESPONSE },
        { role: "user", content: prompt },
    ];

    try {
        const response = await openai.createChatCompletion({
            model: "gpt-3.5-turbo",
            messages,
            temperature: 0.6,
        });

        return response.data;
    } catch (error) {
        console.error("Error fetching from OpenAI:", error);
        // TODO: Handle errors appropriately
    }
}

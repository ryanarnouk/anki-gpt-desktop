import React from 'react';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';

interface Props {
    text: string;
    onTextChange: (state: string) => void;
}

const FileSelector: React.FC<Props> = ({ text, onTextChange }: any) => {
        const selectFile = async () => {
        try {
            const result = await open({
                filters: [
                    { name: 'PDF Files', extensions: ['pdf'] },
                    { name: 'All Files', extensions: ['*'] }
                ],
                multiple: false
            });
           
            const parsed: string = await invoke("parse_pdf", { path: result});

            onTextChange(parsed);
        } catch (error) {
            console.error('Error selecting file:', error);
        }
    };

    const handleTextChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        onTextChange(event.target.value);
    }

    return (
        <div>
            <button onClick={selectFile}>Open PDF</button>
            <br />
            <textarea 
                value={text}
                onChange={handleTextChange}
                rows={10}
                cols={50}
            />
        </div>
    );
};

export default FileSelector;
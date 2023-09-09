
import "./Editor.css";
import { useCallback, useEffect } from "react";
import { useSelector } from "react-redux";
import { viewSelector } from "core/store";
import { EditorContainerId, EditorLog } from "core/editor";
import { Kernel, useKernel } from "core/kernel";
import { Body1 } from "@fluentui/react-components";

import { EditorTree } from "./tree";
import { EditorDropZone } from "./EditorDropZone";

export const EditorRoot: React.FC = () => {
    const kernel = useKernel();
    const { serial, rootPath, openedFile, currentFileSupported } = useSelector(viewSelector);

    useEffect(() => {
        kernel.initEditor();
    } , [kernel]);

    // Disabling this rule as we are using serial to signal when to update
    // A new listDir reference will cause the tree to update
    // /* eslint-disable react-hooks/exhaustive-deps*/
    const listDir = useCallback(async (paths: string[]): Promise<string[]> => {
        const editor = kernel.getEditor();
        if (!editor) {
            return [];
        }
        return editor.listDir(paths);
    }, [serial]);
    return (
        <div id="editor-root">
            {
                rootPath !== undefined ? (
                    <>
                        <div id="editor-tree-container">
                            <EditorTree 
                                rootName={rootPath}
                                listDir={listDir}
                                onClickFile={(path) => {
                            tryWithEditorRef(kernel, 10, (editor) => {
                                editor.openFile(path);
                            });
                                }}
                            />
                        </div>
                        <div id="editor-panel">
                            {
                                openedFile !== undefined ? (
                                    <>
                                        <div id="editor-file-name">
                                    <Body1>{openedFile}</Body1>
                                        </div>
                                        <div id="editor-outer-container">
                                            {
                                                currentFileSupported ? (
                                                <div id={EditorContainerId}></div>
                                                ) : (
                                                        <Body1>The web editor cannot open the file because it is not a text file.</Body1>
                                                )
                                            }
                                        </div>
                                    </>
                                ) : (
                                    <Body1>Click a file to open it</Body1>
                                )
                            }
                        </div>
                            </>
                ) : (
            <EditorDropZone onFileSysCreate={(fileSys) => {
                            tryWithEditorRef(kernel, 10, (editor) => {
                                editor.reset(fileSys);
                            });

            }}/>
                )
            }
        </div>
    );
};

const tryWithEditorRef = (kernel: Kernel, attempts: number, callback: (editor: EditorState) => void) => {
    const doTry = (attempt: number) => {
        if (attempt > attempts) {
            EditorLog.error("Editor not initialized after max attempts");
            return;
        }
        const editor = kernel.getEditor();
        if (!editor) {
            EditorLog.warn("Editor not initialized. Will try again.")
            setTimeout(() => {
                doTry(attempt + 1);
            }, 1000);
            return;
        }
        callback(editor);
    };
    doTry(0);
}
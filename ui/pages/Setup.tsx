import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { CodeBlock } from "react-code-blocks";
import { Loader2 } from "lucide-react";
import { Progress } from "@/components/ui/progress";

function Setup() {

    const [isInitialized, setIsInitialized] = useState(false);
    const [config, setConfig] = useState("");
    const [devkitRoot, setDevkitRoot] = useState("");
    const [editMode, setEditMode] = useState(false);
    const [binariesAvailable, setBinariesAvailable] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [downloadStatus, setDownloadStatus] = useState("");
    const [downloadProgress, setDownloadProgress] = useState(0);

    useEffect(() => {
        checkInitialized();
        invoke("get_devkit_root", {}).then((response) => {
            setDevkitRoot(response as string);
        });
    }, []);

    useEffect(() => {
        if (isInitialized) {
            getConfig();
        }
    }, [isInitialized]);

    function checkInitialized() {
        invoke("is_initialized", {}).then((response) => {
            setIsInitialized(response as boolean);
        });
        invoke("is_yaci_devkit_initialized", {}).then((response) => {
            setBinariesAvailable(response as boolean);
        });
    }

    function getConfig() {
        invoke("get_config", {}).then((response) => {
            setConfig(response as string);
        });
    }

    function init() {
        invoke("init", {}).then((response) => {
            setIsInitialized(true);
            setConfig(response as string);
        });
    }

    function saveConfig() {
        invoke("save_config", { config }).then((response) => {
            setEditMode(false);
            setConfig(response as string);
            checkInitialized();
        });
    }

    function updateDownloadStatus(status: string, current: number, total: number) {
        setDownloadStatus(status);
        setDownloadProgress(Math.round((current / total) * 100));
    }

    function downloadBinaries() {
        setIsLoading(true);
        const download = listen("download-progress", (event: any) => {
            const [current, total] = event.payload as [number, number];
            updateDownloadStatus("Downloading", current, total);
        });
        const zip = listen("unzip-progress", (event: any) => {
            const [current, total] = event.payload as [number, number];
            updateDownloadStatus("Unzipping", current, total);
        });
        const setupComplete = listen("setup-complete", (event: any) => {
            setBinariesAvailable(true);
            setIsLoading(false);
        });
        invoke("download_binaries").catch((err) => {
            console.error(err);
        });

        return () => {
            download.then((fn) => fn());
            zip.then((fn) => fn());
            setupComplete.then((fn) => fn());
        }
    }


    return (
        <main className="container w-full">
            Before using the Cardano Devkit, it must be initialized. This process includes creating a configuration file and downloading the necessary binaries to ensure proper functionality.
            <div className="pt-6">
                {isInitialized ?
                    <Button className="bg-green-500">Initialized</Button> :
                    <Button className="bg-red-500" onClick={() => init()}>Not Initialized</Button>
                }
                <div className="pt-6">

                    {isInitialized &&
                        <div className="space-y-4">
                            Config is saved at: {devkitRoot}
                            <div className="relative rounded-lg shadow-lg">
                                {editMode ?
                                    <textarea className="w-full h-60 p-4" value={config} onChange={(e) => setConfig(e.target.value)} /> :
                                    <CodeBlock customStyle={{ textAlign: 'left' }} text={config} language="json" showLineNumbers={false} />}

                                {/* Button */}
                                <div className="absolute bottom-2 right-2">
                                    {editMode ?
                                        <button
                                            onClick={() => saveConfig()}
                                            className="bg-blue-500 text-white px-3 py-2 text-sm rounded shadow hover:bg-blue-600">
                                            Save
                                        </button> :
                                        <button
                                            onClick={() => setEditMode(true)}
                                            className="bg-blue-500 text-white px-3 py-2 text-sm rounded shadow hover:bg-blue-600">
                                            Edit
                                        </button>
                                    }
                                </div>
                            </div>
                            {binariesAvailable ?
                                <>Binaries Available</> :
                                isLoading ?
                                    <div className="flex flex-col items-center justify-center space-y-2 p-4">
                                        {downloadStatus}
                                        <Progress value={downloadProgress} />
                                    </div> :
                                    <Button onClick={() => downloadBinaries()}>Download Binaries</Button>
                            }
                        </div>
                    }
                </div>
            </div>
        </main >
    );
}

export default Setup;
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { CodeBlock } from "react-code-blocks";
import { Loader2 } from "lucide-react";

function Setup() {

    const [isInitialized, setIsInitialized] = useState(false);
    const [config, setConfig] = useState("");
    const [devkitRoot, setDevkitRoot] = useState("");
    const [editMode, setEditMode] = useState(false);
    const [binariesAvailable, setBinariesAvailable] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

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

    function downloadBinaries() {
        setIsLoading(true);
        invoke("download_binaries", {}).then(() => {
            invoke("is_yaci_devkit_initialized", {}).then((response) => {
                setBinariesAvailable(response as boolean);
            });
            setIsLoading(false);
        });
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
                                        <Loader2 className="h-8 w-8 animate-spin text-blue-500" />
                                        <p className="text-sm text-gray-500">It can take a few minutes...</p>
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
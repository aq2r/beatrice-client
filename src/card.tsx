import { useEffect, useState } from "react";
import { BeatriceModelInfo, RustInvoke } from "./rustInvoke";
import { useAtom, useSetAtom } from "jotai";
import { jotaiAtoms } from "./atoms";

export const ModelCard = (
    { modelInfo }: { modelInfo: BeatriceModelInfo, }
) => {
    const [imageSrc, setImageSrc] = useState("");

    const [selectModel, setSelectModel] = useAtom(jotaiAtoms.selectModel)
    const setSelectSpeakerIdx = useSetAtom(jotaiAtoms.selectSpeakerIdx)

    useEffect(() => {
        const loadImage = async () => {
            const base64Image = await RustInvoke.otherReadImage(
                `${modelInfo.model_path}/${modelInfo.voices[0].portrait_path}`
            );
            setImageSrc(base64Image);
        }
        loadImage();
    }, []);

    return (
        <div
            className={`card ${modelInfo === selectModel && "active"}`}
            onClick={() => {
                setSelectSpeakerIdx(0);
                setSelectModel(modelInfo)
            }}>
            <img src={imageSrc} />
            <div className="card-info">
                {`${modelInfo.name}\n${modelInfo.description}`}
            </div>
        </div >
    )
}

export const SelectDirCard = () => {
    const setAllModelInfo = useSetAtom(jotaiAtoms.allModelInfo);

    return (
        <div className="card card-select-dir" onClick={async () => {
            try {
                const modelInfo = await RustInvoke.Beatrice.searchModel();
                setAllModelInfo(modelInfo);
            } catch (err) {
                if (err !== "SelectCanceled") {
                    console.log(err);
                }
            }
        }}>
            <span>...</span>

            <div className="card-info">
                モデルを読み込むフォルダを選択する
            </div>
        </div>
    )
}

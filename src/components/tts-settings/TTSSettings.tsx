type ttsModelItem = {
    name: string;
    icon_path: string;
    setting_form: React.ReactNode;
    description: string;
};

import kokoroIcon from '@/assets/kokoro.png';
import qwenIcon from '@/assets/qwen-color.png';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog';
import { CircleCheck, Settings } from 'lucide-react';
import React from 'react';
import KokoroSetting from './KokoroSetting';
import QWenSetting from './QWenSetting';

const TTS_MODEL: ttsModelItem[] = [
    {
        name: 'QWen',
        icon_path: qwenIcon,
        description:
            '千问TTS提供流式文本输入与流式音频输出能力，提供多种拟人音色，支持多语种/方言合成，可在同一音色下输出多语种，并能自适应调节语气，流畅处理复杂文本。',
        setting_form: QWenSetting(),
    },
    {
        name: 'Kokoro',
        icon_path: kokoroIcon,
        description:
            'Kokoro 是一款拥有 8200 万个参数的开放式 TTS 模型。尽管其架构轻量级，但它在提供与大型模型相媲美的质量的同时，速度更快、成本效益更高。',
        setting_form: KokoroSetting(),
    },
];
// 使用纯 HTML 和 JavaScript 避免 React hooks
export default function TTSSettings() {
    // let [selected, setSelected] = useState<number>(0);
    return (
        <>
            {TTS_MODEL.map((item, i) => {
                return (
                    <div
                        key={i}
                        className="group bg-muted/50 ring-border relative flex cursor-pointer items-center gap-4 rounded-xl px-4 pt-4 pb-8 hover:shadow-2xs"
                        onClick={() => {}}
                    >
                        <div className="bg-background ring-border m-2 shrink-0 overflow-hidden rounded-md shadow-lg ring-1">
                            <img src={item.icon_path} alt={item.name} className="h-12 w-12" />
                        </div>
                        <div className="flex flex-1 flex-col justify-start gap-1">
                            <p className="text-start font-medium">{item.name}</p>
                            <p className="text-muted-foreground text-start text-sm">
                                {item.description}
                            </p>
                        </div>
                        <div className="absolute top-4 right-4 flex items-center">
                            <CircleCheck
                                className="h-4 w-4 overflow-visible shadow-emerald-300/50 drop-shadow-lg"
                                color="green"
                            />
                        </div>
                        <Dialog>
                            <DialogTrigger>
                                <div className="absolute right-4 bottom-4 flex items-center opacity-0 transition-opacity duration-200 group-hover:opacity-100">
                                    <div className="flex items-center gap-1 hover:text-blue-300">
                                        <Settings className="h-4 w-4" />
                                        设置
                                    </div>
                                </div>
                            </DialogTrigger>
                            <DialogContent>
                                <DialogHeader>
                                    <DialogTitle>{item.name} 设置</DialogTitle>
                                    <DialogDescription></DialogDescription>
                                </DialogHeader>
                                {item.setting_form}
                            </DialogContent>
                        </Dialog>
                    </div>
                );
            })}
        </>
    );
}

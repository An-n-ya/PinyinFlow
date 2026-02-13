import { motion } from 'motion/react';
interface AudioVisualizerProps {
    isPlaying: boolean | undefined;
}
const isPlayingStyle = { width: '16px', scale: 1, display: 'block' };
const isNotPlayingStyle = { width: '0px', scale: 0, display: 'none' };
const AudioVisualizer = ({ isPlaying }: AudioVisualizerProps) => {
    return (
        <motion.svg
            initial={isNotPlayingStyle}
            animate={isPlaying ? isPlayingStyle : isNotPlayingStyle}
            transition={{
                duration: 0.3,
                ease: 'linear',
            }}
            width="16"
            height="16"
            className="text-slate-600"
            viewBox="0 0 16 16"
            fill="currentColor"
            xmlns="http://www.w3.org/2000/svg"
        >
            <style>
                {`
            .bar {
              animation: quiet 1.2s ease-in-out infinite;
              transform-origin: bottom;
            }
            @keyframes quiet {
              0%, 100% { transform: scaleY(0.3); }
              50% { transform: scaleY(1); }
            }
          `}
            </style>

            {/* 四根跳动的柱子 */}
            <rect
                className="bar"
                x="2"
                y="4"
                width="3"
                height="16"
                rx="1.5"
                style={{ animationDuration: '0.8s' }}
            />
            <rect
                className="bar"
                x="8"
                y="4"
                width="3"
                height="16"
                rx="1.5"
                style={{ animationDuration: '0.5s' }}
            />
            <rect
                className="bar"
                x="14"
                y="4"
                width="3"
                height="16"
                rx="1.5"
                style={{ animationDuration: '0.7s' }}
            />
            <rect
                className="bar"
                x="20"
                y="4"
                width="3"
                height="16"
                rx="1.5"
                style={{ animationDuration: '0.6s' }}
            />
        </motion.svg>
    );
};

export default AudioVisualizer;

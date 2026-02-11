const AudioVisualizer = () => {
    const bars = [
        { id: 1, animDuration: '0.6s' },
        { id: 2, animDuration: '0.8s' },
        { id: 3, animDuration: '0.5s' },
        { id: 4, animDuration: '0.7s' },
    ];

    return (
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
            <svg
                width="16"
                height="16"
                className="text-slate-600"
                viewBox="0 0 16 16"
                fill="currentColor"
                xmlns="http://www.w3.org/2000/svg"
            >
                {/* 这里的动画逻辑通过 CSS 控制 */}
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
            </svg>
        </div>
    );
};

export default AudioVisualizer;

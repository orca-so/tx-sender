import { themes } from "prism-react-renderer";

export default {
    title: "Transaction Sender",
    tagline: "Solana transaction sender with priority fees and Jito tips",
    favicon: "https://orca.so/favicon.ico",

    url: "https://orca-so.github.io",
    baseUrl: "/tx-sender/",
    trailingSlash: true,

    organizationName: "orca-so",
    projectName: "tx-sender",
    onBrokenLinks: "ignore",

    markdown: {
        hooks: {
            onBrokenMarkdownLinks: "ignore",
        },
    },

    i18n: {
        defaultLocale: "en",
        locales: ["en"],
    },

    staticDirectories: ["static", "../ts/dist", "../rust/dist"],

    presets: [
        [
            "@docusaurus/preset-classic",
            {
                docs: {
                    routeBasePath: "/",
                    sidebarPath: "./sidebars.js",
                },
                theme: {
                    customCss: "./static/index.css",
                },
            },
        ],
    ],

    themeConfig: {
        navbar: {
            title: "Transaction Sender",
            logo: {
                alt: "Orca Logo",
                src: "https://orca.so/android-chrome-192x192.png",
            },
            items: [
                {
                    href: "/ts/",
                    label: "TypeScript SDK",
                    position: "left",
                    target: "_blank",
                },
                {
                    href: "/rust/orca_tx_sender/",
                    label: "Rust SDK",
                    position: "left",
                    target: "_blank",
                },
                {
                    href: "https://github.com/orca-so/tx-sender",
                    label: "GitHub",
                    position: "right",
                },
            ],
        },
        prism: {
            theme: themes.github,
            darkTheme: themes.dracula,
        },
    },
};

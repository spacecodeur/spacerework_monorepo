import type { TuonoConfig } from 'tuono/config'

const config: TuonoConfig = {
    server: {
        host: '0.0.0.0',
        port: 3000,
    },
    vite: {
        css: {
            preprocessorOptions: {
                scss: {
                    api: 'modern-compiler'
                }
            }
        },
    }
}

export default config

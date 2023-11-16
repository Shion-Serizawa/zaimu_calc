// server.ts
import { serve } from "https://deno.land/std@0.140.0/http/server.ts";

async function handleRequest(request: Request): Promise<Response> {
    const url = new URL(request.url);
    console.log(url.pathname);

    // ルートディレクトリのパスを指定（ファイルの場所によって変更）
    const root = `${Deno.cwd()}`;

    try {
        // ファイルのパスを取得
        let path = url.pathname;
        if (path === '/') {
            path = '/index.html'; // デフォルトでindex.htmlを提供
        }

        // ファイルを読み込む
        const file = await Deno.readFile(`${root}${path}`);
        const contentType = getContentType(path);

        // ファイルをレスポンスとして返す
        return new Response(file, {
            status: 200,
            headers: {
                "content-type": contentType,
            },
        });
    } catch (e) {
        // ファイルが見つからない場合は404エラー
        return new Response("File not found", { status: 404 });
    }
}

function getContentType(path: string): string {
    if (path.endsWith('.html')) {
        return "text/html";
    } else if (path.endsWith('.js')) {
        return "application/javascript";
    } else if (path.endsWith('.json')) {
        return "application/json";
    } else if (path.endsWith('.wasm')) {
        return "application/wasm";
    } else if (path.endsWith('.ico')) {
        return "image/x-icon";
    }
    return "text/plain";
}


console.log("HTTP webserver running. Access it at: http://localhost:8000/");
await serve(handleRequest, { port: 8000 });

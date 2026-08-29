export function validateRequest({ method, url, headers, body }) {
    const trimmedUrl = url.trim();
    if (!trimmedUrl) return { error: "请输入请求地址。" };

    const activeHeaders = headers
        .filter((header) => header.name.trim() || header.value.trim())
        .map((header) => ({
            name: header.name.trim(),
            value: header.value,
        }));

    if (activeHeaders.some((header) => !header.name)) {
        return { error: "每个请求头都需要提供名称。" };
    }

    const requestBody = method !== "GET" ? body.trim() : "";
    if (requestBody) {
        try {
            JSON.parse(requestBody);
        } catch {
            return { error: "请求体必须是有效的 JSON。" };
        }
    }

    return {
        value: {
            method,
            url: trimmedUrl,
            headers: activeHeaders,
            body: requestBody || null,
        },
    };
}

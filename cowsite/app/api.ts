import { CowfilesManagerClient } from "@/api/cowfiles_grpc_pb";
import { Cowfile, Cowfiles, GetCowfileRequest, GetCowfilesRequest, type CowfileDescriptor, Preview, GetPreviewRequest } from "@/api/cowfiles_pb";
import { ServiceError, credentials } from "@grpc/grpc-js";


type Res<T> = (v: T | PromiseLike<T>) => void;
type Rej = (err: any) => void;

function getClient(): CowfilesManagerClient {
    const serverUrl = process.env.SERVER_URL;
    if (!serverUrl) throw new Error(`Missing env SERVER_URL`);
    const client = new CowfilesManagerClient(serverUrl, credentials.createInsecure());
    return client;
}

const callback = <T>(res: Res<T>, rej: Rej) => (err: ServiceError | null, resp: T) => {
    if (err) rej(err);
    else res(resp);
}

export async function getCowfiles(serverId?: string): Promise<CowfileDescriptor[]> {
    const client = getClient();

    const resp: Cowfiles = await new Promise((res, rej) => {
        const req = new GetCowfilesRequest();
        if (serverId) req.setServerId(serverId);
        client.getCowfiles(req, callback(res, rej));
    });

    return resp.getCowfilesList();
}

export async function getCowfile(id: string): Promise<Cowfile> {
    const client = getClient();

    return new Promise((res, rej) => {
        const req = new GetCowfileRequest();
        req.setId(id);
        client.getCowfile(req, callback(res, rej));
    });
}

export async function getPreview(id: string): Promise<Preview> {
    const client = getClient();

    return new Promise((res, rej) => {
        const req = new GetPreviewRequest();
        req.setId(id);
        client.getPreview(req, callback(res, rej));
    });
}

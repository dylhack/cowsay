import { CowfilesManagerClient } from "./cowserve/cowfiles_grpc_pb";
import { ServiceError, credentials } from "@grpc/grpc-js";
import { CowfileData, CowfileDescriptor as ApiCowfileDescriptor, Cowfiles, GetCowfileRequest, GetCowfilesRequest, Preview } from "./cowserve/cowfiles_pb";


type GetFn<T> = (req: GetCowfileRequest, cb: (err: ServiceError | null, resp: T) => void) => void;
type Res<T> = (v: T | PromiseLike<T>) => void;
type Rej = (err: any) => void;

// API Types
export type CowfileDescriptor = ApiCowfileDescriptor.AsObject;

const callback = <T>(res: Res<T>, rej: Rej) => (err: ServiceError | null, resp: T) => {
    if (err) rej(err);
    else res(resp);
}

function getClient(): CowfilesManagerClient {
    const serverUrl = process.env.SERVER_URL;
    if (!serverUrl) throw new Error(`Missing env SERVER_URL`);
    const client = new CowfilesManagerClient(serverUrl, credentials.createInsecure());
    return client;
}

function buildGetReq<T>(fn: GetFn<T>, id: string): Promise<T> {
    const req = new GetCowfileRequest();
    req.setId(id);
    return new Promise((res, rej) => fn.bind(getClient())(req, callback(res, rej)));
}

export const getCowfile = (id: string) => buildGetReq<ApiCowfileDescriptor>(CowfilesManagerClient.prototype.getCowfile, id).then(r => r.toObject());
export const getCowdata = (id: string) => buildGetReq<CowfileData>(CowfilesManagerClient.prototype.getCowdata, id).then(r => r.toObject());
export const getPreview = (id: string) => buildGetReq<Preview>(CowfilesManagerClient.prototype.getPreview, id);
export async function getCowfiles(serverId?: string): Promise<ApiCowfileDescriptor[]> {
    const client = getClient();
    const req = new GetCowfilesRequest();
    if (serverId) req.setServerId(serverId);
    const resp: Cowfiles = await new Promise((res, rej) => client.getCowfiles(req, callback(res, rej)));

    return resp.getCowfilesList();
}

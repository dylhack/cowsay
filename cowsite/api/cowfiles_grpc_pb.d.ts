// package: cowfiles
// file: cowfiles.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as cowfiles_pb from "./cowfiles_pb";

interface ICowfilesManagerService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    saveCowfile: ICowfilesManagerService_ISaveCowfile;
    getCowfiles: ICowfilesManagerService_IGetCowfiles;
    getCowfile: ICowfilesManagerService_IGetCowfile;
    getPreview: ICowfilesManagerService_IGetPreview;
}

interface ICowfilesManagerService_ISaveCowfile extends grpc.MethodDefinition<cowfiles_pb.SaveCowfileRequest, cowfiles_pb.Cowfile> {
    path: "/cowfiles.CowfilesManager/SaveCowfile";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.SaveCowfileRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.SaveCowfileRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.Cowfile>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.Cowfile>;
}
interface ICowfilesManagerService_IGetCowfiles extends grpc.MethodDefinition<cowfiles_pb.GetCowfilesRequest, cowfiles_pb.Cowfiles> {
    path: "/cowfiles.CowfilesManager/GetCowfiles";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetCowfilesRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetCowfilesRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.Cowfiles>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.Cowfiles>;
}
interface ICowfilesManagerService_IGetCowfile extends grpc.MethodDefinition<cowfiles_pb.GetCowfileRequest, cowfiles_pb.Cowfile> {
    path: "/cowfiles.CowfilesManager/GetCowfile";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetCowfileRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetCowfileRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.Cowfile>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.Cowfile>;
}
interface ICowfilesManagerService_IGetPreview extends grpc.MethodDefinition<cowfiles_pb.GetPreviewRequest, cowfiles_pb.Preview> {
    path: "/cowfiles.CowfilesManager/GetPreview";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetPreviewRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetPreviewRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.Preview>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.Preview>;
}

export const CowfilesManagerService: ICowfilesManagerService;

export interface ICowfilesManagerServer extends grpc.UntypedServiceImplementation {
    saveCowfile: grpc.handleUnaryCall<cowfiles_pb.SaveCowfileRequest, cowfiles_pb.Cowfile>;
    getCowfiles: grpc.handleUnaryCall<cowfiles_pb.GetCowfilesRequest, cowfiles_pb.Cowfiles>;
    getCowfile: grpc.handleUnaryCall<cowfiles_pb.GetCowfileRequest, cowfiles_pb.Cowfile>;
    getPreview: grpc.handleUnaryCall<cowfiles_pb.GetPreviewRequest, cowfiles_pb.Preview>;
}

export interface ICowfilesManagerClient {
    saveCowfile(request: cowfiles_pb.SaveCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    getCowfiles(request: cowfiles_pb.GetCowfilesRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    getCowfile(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    getPreview(request: cowfiles_pb.GetPreviewRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    getPreview(request: cowfiles_pb.GetPreviewRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    getPreview(request: cowfiles_pb.GetPreviewRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
}

export class CowfilesManagerClient extends grpc.Client implements ICowfilesManagerClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public saveCowfile(request: cowfiles_pb.SaveCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    public saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    public saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    public getCowfiles(request: cowfiles_pb.GetCowfilesRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    public getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    public getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    public getCowfile(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    public getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    public getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfile) => void): grpc.ClientUnaryCall;
    public getPreview(request: cowfiles_pb.GetPreviewRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    public getPreview(request: cowfiles_pb.GetPreviewRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    public getPreview(request: cowfiles_pb.GetPreviewRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
}

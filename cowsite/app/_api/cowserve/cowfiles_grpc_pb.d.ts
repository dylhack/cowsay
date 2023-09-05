// package: cowfiles
// file: cowfiles.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as cowfiles_pb from "./cowfiles_pb";

interface ICowfilesManagerService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    saveCowfile: ICowfilesManagerService_ISaveCowfile;
    cowsay: ICowfilesManagerService_ICowsay;
    getCowfiles: ICowfilesManagerService_IGetCowfiles;
    getCowfile: ICowfilesManagerService_IGetCowfile;
    getPreview: ICowfilesManagerService_IGetPreview;
    getCowdata: ICowfilesManagerService_IGetCowdata;
}

interface ICowfilesManagerService_ISaveCowfile extends grpc.MethodDefinition<cowfiles_pb.SaveCowfileRequest, cowfiles_pb.CowfileDescriptor> {
    path: "/cowfiles.CowfilesManager/SaveCowfile";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.SaveCowfileRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.SaveCowfileRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.CowfileDescriptor>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.CowfileDescriptor>;
}
interface ICowfilesManagerService_ICowsay extends grpc.MethodDefinition<cowfiles_pb.GetCowsayRequest, cowfiles_pb.CowsayData> {
    path: "/cowfiles.CowfilesManager/Cowsay";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetCowsayRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetCowsayRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.CowsayData>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.CowsayData>;
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
interface ICowfilesManagerService_IGetCowfile extends grpc.MethodDefinition<cowfiles_pb.GetCowfileRequest, cowfiles_pb.CowfileDescriptor> {
    path: "/cowfiles.CowfilesManager/GetCowfile";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetCowfileRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetCowfileRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.CowfileDescriptor>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.CowfileDescriptor>;
}
interface ICowfilesManagerService_IGetPreview extends grpc.MethodDefinition<cowfiles_pb.GetCowfileRequest, cowfiles_pb.Preview> {
    path: "/cowfiles.CowfilesManager/GetPreview";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetCowfileRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetCowfileRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.Preview>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.Preview>;
}
interface ICowfilesManagerService_IGetCowdata extends grpc.MethodDefinition<cowfiles_pb.GetCowfileRequest, cowfiles_pb.CowfileData> {
    path: "/cowfiles.CowfilesManager/GetCowdata";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<cowfiles_pb.GetCowfileRequest>;
    requestDeserialize: grpc.deserialize<cowfiles_pb.GetCowfileRequest>;
    responseSerialize: grpc.serialize<cowfiles_pb.CowfileData>;
    responseDeserialize: grpc.deserialize<cowfiles_pb.CowfileData>;
}

export const CowfilesManagerService: ICowfilesManagerService;

export interface ICowfilesManagerServer extends grpc.UntypedServiceImplementation {
    saveCowfile: grpc.handleUnaryCall<cowfiles_pb.SaveCowfileRequest, cowfiles_pb.CowfileDescriptor>;
    cowsay: grpc.handleUnaryCall<cowfiles_pb.GetCowsayRequest, cowfiles_pb.CowsayData>;
    getCowfiles: grpc.handleUnaryCall<cowfiles_pb.GetCowfilesRequest, cowfiles_pb.Cowfiles>;
    getCowfile: grpc.handleUnaryCall<cowfiles_pb.GetCowfileRequest, cowfiles_pb.CowfileDescriptor>;
    getPreview: grpc.handleUnaryCall<cowfiles_pb.GetCowfileRequest, cowfiles_pb.Preview>;
    getCowdata: grpc.handleUnaryCall<cowfiles_pb.GetCowfileRequest, cowfiles_pb.CowfileData>;
}

export interface ICowfilesManagerClient {
    saveCowfile(request: cowfiles_pb.SaveCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    cowsay(request: cowfiles_pb.GetCowsayRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowsayData) => void): grpc.ClientUnaryCall;
    cowsay(request: cowfiles_pb.GetCowsayRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowsayData) => void): grpc.ClientUnaryCall;
    cowsay(request: cowfiles_pb.GetCowsayRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowsayData) => void): grpc.ClientUnaryCall;
    getCowfiles(request: cowfiles_pb.GetCowfilesRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    getCowfile(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    getPreview(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    getPreview(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    getPreview(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    getCowdata(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileData) => void): grpc.ClientUnaryCall;
    getCowdata(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileData) => void): grpc.ClientUnaryCall;
    getCowdata(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileData) => void): grpc.ClientUnaryCall;
}

export class CowfilesManagerClient extends grpc.Client implements ICowfilesManagerClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public saveCowfile(request: cowfiles_pb.SaveCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    public saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    public saveCowfile(request: cowfiles_pb.SaveCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    public cowsay(request: cowfiles_pb.GetCowsayRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowsayData) => void): grpc.ClientUnaryCall;
    public cowsay(request: cowfiles_pb.GetCowsayRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowsayData) => void): grpc.ClientUnaryCall;
    public cowsay(request: cowfiles_pb.GetCowsayRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowsayData) => void): grpc.ClientUnaryCall;
    public getCowfiles(request: cowfiles_pb.GetCowfilesRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    public getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    public getCowfiles(request: cowfiles_pb.GetCowfilesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Cowfiles) => void): grpc.ClientUnaryCall;
    public getCowfile(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    public getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    public getCowfile(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileDescriptor) => void): grpc.ClientUnaryCall;
    public getPreview(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    public getPreview(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    public getPreview(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.Preview) => void): grpc.ClientUnaryCall;
    public getCowdata(request: cowfiles_pb.GetCowfileRequest, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileData) => void): grpc.ClientUnaryCall;
    public getCowdata(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileData) => void): grpc.ClientUnaryCall;
    public getCowdata(request: cowfiles_pb.GetCowfileRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: cowfiles_pb.CowfileData) => void): grpc.ClientUnaryCall;
}

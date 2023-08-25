// package: cowfiles
// file: cowfiles.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class SaveCowfileRequest extends jspb.Message { 
    getServerId(): string;
    setServerId(value: string): SaveCowfileRequest;
    getName(): string;
    setName(value: string): SaveCowfileRequest;
    getUploaderId(): string;
    setUploaderId(value: string): SaveCowfileRequest;

    hasAuthor(): boolean;
    clearAuthor(): void;
    getAuthor(): string | undefined;
    setAuthor(value: string): SaveCowfileRequest;
    getData(): string;
    setData(value: string): SaveCowfileRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): SaveCowfileRequest.AsObject;
    static toObject(includeInstance: boolean, msg: SaveCowfileRequest): SaveCowfileRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: SaveCowfileRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): SaveCowfileRequest;
    static deserializeBinaryFromReader(message: SaveCowfileRequest, reader: jspb.BinaryReader): SaveCowfileRequest;
}

export namespace SaveCowfileRequest {
    export type AsObject = {
        serverId: string,
        name: string,
        uploaderId: string,
        author?: string,
        data: string,
    }
}

export class GetCowfilesRequest extends jspb.Message { 

    hasServerId(): boolean;
    clearServerId(): void;
    getServerId(): string | undefined;
    setServerId(value: string): GetCowfilesRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetCowfilesRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetCowfilesRequest): GetCowfilesRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetCowfilesRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetCowfilesRequest;
    static deserializeBinaryFromReader(message: GetCowfilesRequest, reader: jspb.BinaryReader): GetCowfilesRequest;
}

export namespace GetCowfilesRequest {
    export type AsObject = {
        serverId?: string,
    }
}

export class GetCowfileRequest extends jspb.Message { 
    getId(): string;
    setId(value: string): GetCowfileRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetCowfileRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetCowfileRequest): GetCowfileRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetCowfileRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetCowfileRequest;
    static deserializeBinaryFromReader(message: GetCowfileRequest, reader: jspb.BinaryReader): GetCowfileRequest;
}

export namespace GetCowfileRequest {
    export type AsObject = {
        id: string,
    }
}

export class GetPreviewRequest extends jspb.Message { 
    getId(): string;
    setId(value: string): GetPreviewRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetPreviewRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetPreviewRequest): GetPreviewRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetPreviewRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetPreviewRequest;
    static deserializeBinaryFromReader(message: GetPreviewRequest, reader: jspb.BinaryReader): GetPreviewRequest;
}

export namespace GetPreviewRequest {
    export type AsObject = {
        id: string,
    }
}

export class Cowfile extends jspb.Message { 
    getId(): string;
    setId(value: string): Cowfile;

    hasServerId(): boolean;
    clearServerId(): void;
    getServerId(): string | undefined;
    setServerId(value: string): Cowfile;
    getName(): string;
    setName(value: string): Cowfile;
    getAuthor(): string;
    setAuthor(value: string): Cowfile;
    getUploaderId(): string;
    setUploaderId(value: string): Cowfile;
    getData(): string;
    setData(value: string): Cowfile;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Cowfile.AsObject;
    static toObject(includeInstance: boolean, msg: Cowfile): Cowfile.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Cowfile, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Cowfile;
    static deserializeBinaryFromReader(message: Cowfile, reader: jspb.BinaryReader): Cowfile;
}

export namespace Cowfile {
    export type AsObject = {
        id: string,
        serverId?: string,
        name: string,
        author: string,
        uploaderId: string,
        data: string,
    }
}

export class CowfileDescriptor extends jspb.Message { 
    getId(): string;
    setId(value: string): CowfileDescriptor;

    hasServerId(): boolean;
    clearServerId(): void;
    getServerId(): string | undefined;
    setServerId(value: string): CowfileDescriptor;
    getName(): string;
    setName(value: string): CowfileDescriptor;
    getAuthor(): string;
    setAuthor(value: string): CowfileDescriptor;
    getUploaderId(): string;
    setUploaderId(value: string): CowfileDescriptor;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CowfileDescriptor.AsObject;
    static toObject(includeInstance: boolean, msg: CowfileDescriptor): CowfileDescriptor.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CowfileDescriptor, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CowfileDescriptor;
    static deserializeBinaryFromReader(message: CowfileDescriptor, reader: jspb.BinaryReader): CowfileDescriptor;
}

export namespace CowfileDescriptor {
    export type AsObject = {
        id: string,
        serverId?: string,
        name: string,
        author: string,
        uploaderId: string,
    }
}

export class Cowfiles extends jspb.Message { 
    clearCowfilesList(): void;
    getCowfilesList(): Array<CowfileDescriptor>;
    setCowfilesList(value: Array<CowfileDescriptor>): Cowfiles;
    addCowfiles(value?: CowfileDescriptor, index?: number): CowfileDescriptor;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Cowfiles.AsObject;
    static toObject(includeInstance: boolean, msg: Cowfiles): Cowfiles.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Cowfiles, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Cowfiles;
    static deserializeBinaryFromReader(message: Cowfiles, reader: jspb.BinaryReader): Cowfiles;
}

export namespace Cowfiles {
    export type AsObject = {
        cowfilesList: Array<CowfileDescriptor.AsObject>,
    }
}

export class Preview extends jspb.Message { 
    getData(): Uint8Array | string;
    getData_asU8(): Uint8Array;
    getData_asB64(): string;
    setData(value: Uint8Array | string): Preview;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Preview.AsObject;
    static toObject(includeInstance: boolean, msg: Preview): Preview.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Preview, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Preview;
    static deserializeBinaryFromReader(message: Preview, reader: jspb.BinaryReader): Preview;
}

export namespace Preview {
    export type AsObject = {
        data: Uint8Array | string,
    }
}

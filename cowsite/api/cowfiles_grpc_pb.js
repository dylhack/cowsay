// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var cowfiles_pb = require('./cowfiles_pb.js');

function serialize_cowfiles_Cowfile(arg) {
  if (!(arg instanceof cowfiles_pb.Cowfile)) {
    throw new Error('Expected argument of type cowfiles.Cowfile');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cowfiles_Cowfile(buffer_arg) {
  return cowfiles_pb.Cowfile.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_cowfiles_Cowfiles(arg) {
  if (!(arg instanceof cowfiles_pb.Cowfiles)) {
    throw new Error('Expected argument of type cowfiles.Cowfiles');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cowfiles_Cowfiles(buffer_arg) {
  return cowfiles_pb.Cowfiles.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_cowfiles_GetCowfileRequest(arg) {
  if (!(arg instanceof cowfiles_pb.GetCowfileRequest)) {
    throw new Error('Expected argument of type cowfiles.GetCowfileRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cowfiles_GetCowfileRequest(buffer_arg) {
  return cowfiles_pb.GetCowfileRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_cowfiles_GetCowfilesRequest(arg) {
  if (!(arg instanceof cowfiles_pb.GetCowfilesRequest)) {
    throw new Error('Expected argument of type cowfiles.GetCowfilesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cowfiles_GetCowfilesRequest(buffer_arg) {
  return cowfiles_pb.GetCowfilesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_cowfiles_SaveCowfileRequest(arg) {
  if (!(arg instanceof cowfiles_pb.SaveCowfileRequest)) {
    throw new Error('Expected argument of type cowfiles.SaveCowfileRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_cowfiles_SaveCowfileRequest(buffer_arg) {
  return cowfiles_pb.SaveCowfileRequest.deserializeBinary(new Uint8Array(buffer_arg));
}


// The greeting service definition.
var CowfilesManagerService = exports.CowfilesManagerService = {
  saveCowfile: {
    path: '/cowfiles.CowfilesManager/SaveCowfile',
    requestStream: false,
    responseStream: false,
    requestType: cowfiles_pb.SaveCowfileRequest,
    responseType: cowfiles_pb.Cowfile,
    requestSerialize: serialize_cowfiles_SaveCowfileRequest,
    requestDeserialize: deserialize_cowfiles_SaveCowfileRequest,
    responseSerialize: serialize_cowfiles_Cowfile,
    responseDeserialize: deserialize_cowfiles_Cowfile,
  },
  getCowfiles: {
    path: '/cowfiles.CowfilesManager/GetCowfiles',
    requestStream: false,
    responseStream: false,
    requestType: cowfiles_pb.GetCowfilesRequest,
    responseType: cowfiles_pb.Cowfiles,
    requestSerialize: serialize_cowfiles_GetCowfilesRequest,
    requestDeserialize: deserialize_cowfiles_GetCowfilesRequest,
    responseSerialize: serialize_cowfiles_Cowfiles,
    responseDeserialize: deserialize_cowfiles_Cowfiles,
  },
  getCowfile: {
    path: '/cowfiles.CowfilesManager/GetCowfile',
    requestStream: false,
    responseStream: false,
    requestType: cowfiles_pb.GetCowfileRequest,
    responseType: cowfiles_pb.Cowfile,
    requestSerialize: serialize_cowfiles_GetCowfileRequest,
    requestDeserialize: deserialize_cowfiles_GetCowfileRequest,
    responseSerialize: serialize_cowfiles_Cowfile,
    responseDeserialize: deserialize_cowfiles_Cowfile,
  },
};

exports.CowfilesManagerClient = grpc.makeGenericClientConstructor(CowfilesManagerService);

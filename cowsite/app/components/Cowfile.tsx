'use client';
import type { CowfileDescriptor } from "@/api/cowfiles_pb";
import Image from "next/image";


export default function Cowfile(props: CowfileDescriptor.AsObject) {
    return (
        <div className="rounded aspect-square w-80 bg-slate-500 shadow-md container mx-auto px-4">
            <h1>{props.name}</h1>
            <p>by {props.author}</p>
            <Image alt="bruh" src={`/api/preview/${props.id}`} width={200} height={200}></Image>
            <a href={`/api/download/${props.id}`} download="file">Download</a>
        </div>
    );
}

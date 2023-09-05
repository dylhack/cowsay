"use client";
import Image from "next/image";
import { useState } from "react";
import Modal from "@/app/_components/Modal";

type Props = {
    id: string;
    width: number;
    height: number;
    className?: string;
}

export default function Preview({ id, width, height, className }: Props) {
    const [isOpen, setIsOpen] = useState(false);
    const imgUrl = '/api/preview/' + id;
    const onClick = () => setIsOpen(!isOpen);

    return (
        <>
            <Modal isOpen={isOpen} dismiss={() => setIsOpen(false)}>
                <Image src={imgUrl} width={500} height={500} alt="Character preview modal" />
            </Modal>
            <Image className={className} src={imgUrl} width={width} height={height} alt="Character preview" onClick={onClick} />
        </>
    );
}

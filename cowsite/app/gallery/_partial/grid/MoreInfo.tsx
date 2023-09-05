"use client";
import Button from "@/app/_components/Button";
import { useRouter } from "next/navigation";
import styles from './styles.module.css';

type Props = {
    cid: string;
}

export default function MoreInfo({ cid }: Props) {
    const open = () => router.push(`/gallery/${cid}`);
    const router = useRouter();

    return (
        <Button style="primary" onClick={open} className={styles['more-info']}>
            i
        </Button>
    );
}

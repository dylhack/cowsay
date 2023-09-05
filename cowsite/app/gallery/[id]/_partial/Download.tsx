'use client';
import Button from "@/app/_components/Button";
import styles from './styles.module.css';

type Props = {
    id: string;
}

export default function Download({ id }: Props) {
    const download = () => window.location.replace(`/api/download/${id}`);

    return (
        <Button className={styles['download']} style="primary" onClick={download}>
            Download
        </Button>
    );
}

import { CowfileDescriptor } from "@/app/_api/cowserve";
import styles from './styles.module.css';
import { fixCasing } from "@/app/_util/strings";
import type { ReactNode } from "react";
import Preview from "./Preview";

type Props = {
    cowfile: CowfileDescriptor;
    actions: ReactNode[];
}

export default function Card({ actions, cowfile }: Props) {
    const actionStyle = actions.length > 1 ? styles['card-actions'] : styles['card-action'];

    return (
        <div className={styles['card']}>
            <div className="flex flex-col w-100">
                <div className={styles['card-title']}>
                    <h1>{fixCasing(cowfile.name)}</h1>
                    <h2>{cowfile.author}</h2>
                </div>
                <Preview className={styles['card-preview']} id={cowfile.id} width={200} height={200} />
            </div>
            <div className={actionStyle}>
                { ...actions }
            </div>
        </div>
    );
}

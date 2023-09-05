'use client';
import styles from './ModalDialogue.module.css';
import { ReactElement, ReactNode } from 'react';


export type Modal = {
    modal: ReactElement;
    onSuccess?: () => void;
    onFailure?: () => void;
}

type Props = {
    modal?: Modal;
    reset: () => void;
}

export default function ModalDialogue({ modal, reset }: Props) {
    const onClose = () => { 
        modal?.onFailure && modal.onFailure();
        reset();
    }

    return (
        <div>
            {modal && (<div className={styles.dialogue} onClick={onClose}>
                {modal.modal}
            </div>)}
        </div>
    );
}

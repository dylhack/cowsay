"use client";
import React from 'react';
import styles from './Modal.module.css';

type Props = {
    isOpen: boolean;
    dismiss: () => void;
    children: React.ReactNode;
}

export default function Modal({ isOpen, children, dismiss }: Props) {
    return (
        (isOpen && <div className={styles['modal']} onClick={dismiss}>
            {children}
        </div>)
    );
}

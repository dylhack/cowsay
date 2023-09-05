"use client";
import styles from './Button.module.css';

type Props = {
    onClick?: () => void;
    children?: React.ReactNode;
    style?: Style;
    disabled?: boolean;
    className?: string;
}

type Style = 'primary' | 'secondary' | 'danger';

const classes = [styles['button-normal'], styles['button-secondary'], styles['button-primary'], styles['button-danger'] ];

const getStyle = (style: string): string => {
    let res = 0;
    switch (style) {
        case 'primary':
            res = 2;
            break;
        case 'danger':
            res = 3;
            break;
        case 'secondary':
        default:
            res = 0;
    }

    return `${styles['button']} ${classes[res]}`;
}

export default function Button({ onClick, children, style, disabled, className }: Props) {
    const btnStyle = `${getStyle(disabled ? 'secondary' : style ?? '')} ${className}`;
    return (
        <button className={btnStyle} disabled={disabled} onClick={onClick}>
            {children}
        </button>
    )
}

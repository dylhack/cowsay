import Button from "@/app/_components/Button";
import styles from './styles.module.css';

type Props = {
    claimed?: boolean;
}

export default function Claim({ claimed }: Props) {
    const style = claimed ? 'primary' : 'secondary';
    return (
        <Button style={style} className={styles['claim']}>
            {claimed ? 'Me no Want' : 'Me Want'}
        </Button>
    )
}

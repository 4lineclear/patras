import { Link } from "wouter";
import Context from "../Context";
import styles from "./header.module.scss";
// import "./header.module.scss"

/**
 * The optional login buttons
 */
const Buttons = () => (
  <div>
    <span className={styles.linkContainer}>
      <Link href="log-in">
        <span className={styles.linkText}>Log In</span>
      </Link>
    </span>
    <span className={styles.linkContainer}>
      <Link href="sign-up">
        <span className={styles.linkText}>Sign Up</span>
      </Link>
    </span>
  </div>
);

const Header = () => {
  const context = Context;
  return (
    <header>
      <Link id={styles.titleHolder} href="/">
        <span className={styles.titleLarge}>Pa</span>
        <span className={styles.titleSmall}>per</span>
        <span className={styles.titleLarge}>Tra</span>
        <span className={styles.titleSmall}>der</span>
        <span className={styles.titleLarge}>S</span>
        <span className={styles.titleSmall}>olo</span>
      </Link>
      {context.logged_in ? null : <Buttons />}
    </header>
  );
};

export default Header;

import { Link } from "wouter";
import styles from "./index.module.scss";

const GithubLink = (
  <a target="_blank" href="https://github.com/4lineclear/patras">
    Github
  </a>
);

export default function Index() {
  return (
    <div id={styles.page}>
      <p>PaTraS is a hyper minimalist paper trading service.</p>
      <Link id={styles.signUp} href="sign-up">
        <span className={styles.linkText}>Sign Up Here</span>
      </Link>
      <p>See the {GithubLink}.</p>
    </div>
  );
}

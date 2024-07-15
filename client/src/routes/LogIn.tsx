import { ChangeEvent, FormEvent, useContext, useState } from "react";
import styles from "./log-in.module.scss";
import { Context } from "../Context";
import { useLocation } from "wouter";

interface FormInfo {
  username: string;
  password: string;
}

export default function LogIn() {
  const context = useContext(Context);
  const [, setLocation] = useLocation();
  const [showPass, setShowPass] = useState(false);
  const [info, setInfo] = useState<FormInfo>({ username: "", password: "" });
  const infoChange = (event: ChangeEvent<HTMLInputElement>) => {
    setInfo({ ...info, [event.target.name]: event.target.value });
  };
  const [attempts, setAttempts] = useState(0);
  const [response, setResponse] = useState<Response | null>(null);

  const logIn = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const res = await fetch("/api/log-in", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(info),
    });

    if (res.status == 200) {
      context.auth.setLogin("logged-in");
      setLocation("/");
    }
    setResponse(res);
    setAttempts(attempts + 1);
  };

  const infoBarText = () => {
    switch (response?.status) {
      case 401:
        return "Incorrect username or password";
      default:
        return "";
    }
  };

  if (context.auth.login == "logged-in") {
    return (
      <div>
        <h1>User is already logged in</h1>
      </div>
    );
  }

  return (
    <div id={styles.page}>
      <form id={styles.form} onSubmit={logIn}>
        <h2>Login to Patras</h2>
        <p>Complete the form below to login</p>

        <div id={styles.usernameLabelDiv}>
          <label className={styles.label} htmlFor={styles.usernameInput}>
            Username:
          </label>
        </div>
        <input
          id={styles.usernameInput}
          className={styles.input}
          name="username"
          type="text"
          autoComplete="username"
          value={info.username}
          onChange={infoChange}
        />
        <div id={styles.passwordLabelDiv}>
          <label className={styles.label} htmlFor={styles.passwordInput}>
            Password:
          </label>
          <input
            id={styles.showPasswordInput}
            type="checkbox"
            checked={showPass}
            tabIndex={-1}
            onChange={() => setShowPass((prev) => !prev)}
          />
          <span id={styles.showPasswordSpan}>Show</span>
        </div>
        <input
          id={styles.passwordInput}
          className={styles.input}
          name="password"
          type={showPass ? "text" : "password"}
          autoComplete="password"
          value={info.password}
          onChange={infoChange}
        />
        <div id={styles.infoBar}>{infoBarText()}</div>
        <input id={styles.submitButton} type="submit" value="Log In" />
      </form>
    </div>
  );
}

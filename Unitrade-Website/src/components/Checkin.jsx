function Checkin() {
    return (
        <div className="checking">
            <div className="checkin-container">
                <div className="login-logo">
                <img src="src/assets/Unitradelogo2.png" alt="UniTrade" className="unitradelogo"/>
                </div>
                <h1>Welcome Back!</h1>
                <h2>Log in into your account</h2>
                <div className="inputs">
                    <div className="contact">
                        <div>
                            <h3>Phone number</h3>
                        </div>
                        <div className="input">
                        <input type="image text" src="src/assets/Name.png"placeholder="Phone number" />
                        </div>
                    </div>
                    <div className="password">
                        <h3>Password</h3>
                        <div className="input">
                        <input type="text" placeholder="Password" />
                        </div>
                    </div>
                    <div className="checkin-btn">
                                    <h1>
                                        <img src="src/assets/Login.png" alt="Login" />
                                        Log In
                                    </h1>
                    </div>
                </div>
            </div>
            <div className="signup-banner">
                <div className="welcome">
                    <h1> Welcome to</h1>
                    <h2> UniTrade</h2>
                    <h3> For better trading</h3>
                </div>
            </div>
            
        </div>
    )
}
export default Checkin
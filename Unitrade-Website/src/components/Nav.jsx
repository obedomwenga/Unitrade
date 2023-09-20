import React from 'react'

 function Nav() {
    return (
        <div className='navbarcontainer'>
          <div className="navbar">
          <div className="logo">
            <img src="src\assets\Unitradelogo.png" alt="UniTrade" /> 
            <h1>UniTrade</h1>
          </div>
          <nav className='navigation'>
            <ul>
                <li><a href="" rel="noreferrer noopener" target="_blank">Home</a></li>
                <li><a href="" rel="noreferrer noopener" target="_blank">About</a></li>
                <li><a href=""rel="noreferrer noopener" target="_blank">Shop</a></li>
                <li><a href=""rel="noreferrer noopener" target="_blank">Contact</a></li>
            </ul>
          </nav>
            <div className='navbtn'>
                <button type='button'>Join our platform</button>
            </div>
          </div>
        </div>

      );
}
export default Nav
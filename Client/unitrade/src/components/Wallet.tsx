'use client'
import { useEffect,useState,useRef,Fragment } from "react";
import SocialLogin from "@biconomy/web3-auth";
import { ethers, providers } from "ethers";
import { ChainId } from "@biconomy/core-types";
import { BiconomySmartAccount, BiconomySmartAccountConfig } from "@biconomy/account";
import { Bundler } from "@biconomy/bundler";
import { bundler,paymaster } from "@/constants/constants";
import Transfer from "./Transfer";

export default function Wallet(){
    const sdkRef = useRef<SocialLogin | null>(null);
    const [interval,enableInterval] = useState<boolean>(false);
    const [loading,setLoading] = useState<boolean>(false);
    const [,setProvider] = useState<providers.Web3Provider>();
    const [smartAccount,setSmartAccount] = useState<BiconomySmartAccount>();
    //setupsmartAccount
const setUpSmartAccount = async ()=>{
  try{
      if(!sdkRef.current?.provider)return;
      sdkRef.current?.hideWallet();
      setLoading(true);

      let web3Provider = new ethers.providers.Web3Provider(
          sdkRef.current?.provider
      );
      setProvider(web3Provider);
      const config: BiconomySmartAccountConfig ={
          signer: web3Provider.getSigner(),
          chainId: ChainId.POLYGON_MUMBAI,
          bundler: bundler,
          paymaster: paymaster,

      };
      const smartAccount = new BiconomySmartAccount(config);
      await smartAccount.init();
      setSmartAccount(smartAccount);

  }catch(error){
      console.log("the error for smartAccount", error);
  }
  setLoading(false);
}
  
    //login
    const login = async()=>{
if(!sdkRef.current){
    const socialLoginSDk = new SocialLogin();
    await socialLoginSDk.init({
        chainId:ethers.utils.hexValue(ChainId.POLYGON_MUMBAI).toString(),
        network: 'testnet',
    });
    sdkRef.current = socialLoginSDk;
}
if(!sdkRef.current.provider){
    sdkRef.current.showWallet();
    enableInterval(true);
}
else{
  setUpSmartAccount();
    console.log("yollow sdk init");
}
    }



    const logOut = async()=>{
        await sdkRef.current?.logout();
        sdkRef.current?.hideWallet();
        setSmartAccount(undefined);
        enableInterval(false);
    }
    useEffect(()=>{
        let configureLogin: NodeJS.Timeout | undefined;
        if(interval){
            configureLogin = setInterval(()=>{
                if(!!sdkRef.current?.provider){
                    setUpSmartAccount();
                    clearInterval(configureLogin);
                }
            },1000);
        }
    }, [interval]);


    return (
        <Fragment>
          {/* Logout Button */}
          {smartAccount && (
            <button
              onClick={logOut}
              className="absolute right-0 m-3 rounded-lg bg-gradient-to-r from-green-400 to-blue-500 px-4 py-2 font-medium transition-all hover:from-green-500 hover:to-blue-600 "
            >
              Logout
            </button>
          )}
    
          <div className="m-auto flex h-screen flex-col items-center justify-center gap-10 bg-gray-950">
            <h1 className=" text-4xl text-gray-50 font-bold tracking-tight lg:text-5xl">
              Send ERC20 using ERC20
            </h1>
    
            {/* Login Button */}
            {!smartAccount && !loading && (
              <button
                onClick={login}
                className="mt-10 rounded-lg bg-gradient-to-r from-green-400 to-blue-500 px-4 py-2 font-medium  transition-colors hover:from-green-500 hover:to-blue-600"
              >
                Login
              </button>
            )}
    
            {/* Loading state */}
            {loading && <p>Loading account details...</p>}
    
            {smartAccount && (
              <Fragment>{<Transfer smartAccount={smartAccount}/>}</Fragment>
            )}
          </div>
        </Fragment>
      );
}

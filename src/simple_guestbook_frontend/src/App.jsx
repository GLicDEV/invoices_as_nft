import * as React from 'react';
import { simple_guestbook, createActor, canisterId } from "../../declarations/simple_guestbook"
import { invoice_canister } from "../../declarations/invoice_canister"

import { AuthClient } from "@dfinity/auth-client"

import { useEffect } from 'react'



const App = () => {
    const [entries, setEntries] = React.useState([]);
    const [pending, setPending] = React.useState(false);
    const textRef = React.useRef();

    const [actor, setActor] = React.useState(simple_guestbook)


    const handleSubmit = async (e) => {
        e.preventDefault();
        if (pending) return;
        setPending(true);
        const text = textRef.current.value.toString();

        // Interact with hello actor, calling the greet method
        const greeting = await actor.add(text);

        // console.log(greeting)

        setPending(false);
        return false;
    }

    const handleLogin = async (e) => {
        e.preventDefault();

        let authClient = await AuthClient.create()
        let zz = await authClient.login({
            onSuccess: () => {
                const identity = authClient.getIdentity();

                console.log(identity)

                const ag = createActor(canisterId, { agentOptions: { identity: identity } })
                setActor(ag);

                console.log(ag)
            }
        });



    }

    useEffect(() => {
        const interval = setInterval(() => {
            const fetchMetrics = async () => {
                const data = await actor.getAll();
                setEntries(data);
                // console.log(data)
            }
            fetchMetrics();
        }, 1000);
        return () => clearInterval(interval);
    }, []);

    return (
        <>
            <div className="columns is-mobile">
                <div className="column has-background-info">
                    <button onClick={handleLogin}>Login</button>
                    <UserStatus actor={actor} />
                </div>
            </div>
            <div className="columns is-mobile">
                <div className="column is-half is-offset-one-quarter has-background-info-light">
                    <form onSubmit={handleSubmit}>
                        <div className="has-text-centered m-1">Add an entry to our guestbook:</div>
                        <textarea className="textarea" placeholder="e.g. Hello world" ref={textRef}></textarea>
                        <div className="has-text-centered m-1">
                            <button disabled={pending} className="button is-primary" id="clickMeBtn" type="submit">Submit</button>
                        </div>
                    </form>
                </div>
            </div>
            <div className="columns is-mobile ">


                <div className="column is-half is-offset-one-quarter has-background-white-bis ">
                    {entries.map(entry => <Entry entry={entry} />)}
                </div>

            </div>
        </>
    )
}

export default App;


export const Entry = (props) => {
    return (
        <>
            <div className="box has-text-centered has-background-white-ter">
                <div className="">
                    <label className="label">{props.entry.author.toString()}</label>
                    <div className="has-background-grey-lighter">
                        {props.entry.text?.toString()}
                    </div>
                    <p className="help">{Object.keys(props.entry.status).toString()}</p>
                </div>
            </div>
        </>
    )
}

import React from 'react'

export const UserStatus = ({ actor }) => {

    const [userStatus, setUserStatus] = React.useState({});
    const [pending, setPending] = React.useState(false);
    const [confirmResponse, setConfirmResponse] = React.useState("")
    const textRef = React.useRef();

    const [upgradeResponse, setUpgradeResponse] = React.useState("");
    const [invoiceResponse, setInvoiceResponse] = React.useState("")

    const handlePremium = async (e) => {
        e.preventDefault();

        const data = await actor.upgradePremium();

        setUpgradeResponse(data.toString());
        console.log(data)
    }

    const handleUltimate = async (e) => {
        e.preventDefault();

        const data = await actor.upgradeUltimate();

        setUpgradeResponse(data.toString());
        console.log(data)
    }

    const handleConfirmInvoice = async (e) => {
        e.preventDefault();

        const data = await invoice_canister.checkPayment(upgradeResponse)
        console.log(data)
        setInvoiceResponse(data.toString());

    }

    const handleConfirmApp = async (e) => {
        e.preventDefault();

        const data = await actor.verifyPremium();

        console.log(data)
        setConfirmResponse(data);

    }

    // useEffect(() => {
    //     const interval = setInterval(() => {
    //         const fetchMetrics = async () => {
    //             const data = await actor.getUserDetails();
    //             setUserStatus(data);
    //             // console.log(data)
    //         }
    //         fetchMetrics();
    //     }, 1000);
    //     return () => clearInterval(interval);
    // }, []);

    return (
        <>
            <div>
                <button onClick={
                    async () => {
                        const data = await actor.getUserDetails();
                        setUserStatus(data);
                    }
                } className="m-1" id="clickMeBtn">Update Status</button>
            </div>
            {
                userStatus.hasOwnProperty('status') &&
                <>
                    <div>Authenticated as: <strong>{userStatus.principal.toString()}</strong></div>
                    <div>Status: <strong>{Object.keys(userStatus.status).toString()}</strong></div>
                    <div>
                        <button onClick={handlePremium} className="m-1" id="clickMeBtn">Request Premium</button>
                    </div>

                    {

                        upgradeResponse.length > 0 &&
                        <>
                            <div>
                                Please pay <strong>0.001 ICP</strong> to this address:
                            </div>
                            <div>
                                <strong>
                                    {upgradeResponse}
                                </strong>
                            </div>

                            <div>
                                <button onClick={handleConfirmInvoice} className="m-1" id="clickMeBtn">Confirm with Invoice Canister</button>
                                <button onClick={handleConfirmApp} className="m-1" id="clickMeBtn">Confirm with dApp Canister</button>
                            </div>
                            <div>
                                {invoiceResponse}
                            </div>
                            <div>
                                {confirmResponse}
                            </div>
                        </>
                    }


                </>

            }
        </>
    )
}

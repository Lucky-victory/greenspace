import { ThirdwebProvider } from "@thirdweb-dev/react";
import { AppWrapper } from "@/context/state";
import { Provider as ReduxProvider } from "react-redux";
import store from "@/state/store";
import { RainbowKitSiweNextAuthProvider } from "@rainbow-me/rainbowkit-siwe-next-auth";
import { RainbowKitProvider } from "@rainbow-me/rainbowkit";
import { SessionProvider } from "next-auth/react";
import type { Session } from "next-auth";
import { AppProps } from "next/app";
import { WagmiProvider } from "wagmi";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";

const queryClient = new QueryClient();
function AppProviders({ children }: { children: React.ReactNode }) {
  return (
    // <WagmiConfig config={config}>

    <AppWrapper>
      <ReduxProvider store={store}>
        <ThirdwebProvider
          clientId="7d6dd3b28e4d16bb007c78b1f6c90b04"
          activeChain="sepolia"
        >
          {children}
        </ThirdwebProvider>
      </ReduxProvider>
    </AppWrapper>
    // </WagmiConfig>
  );
}

export default AppProviders;

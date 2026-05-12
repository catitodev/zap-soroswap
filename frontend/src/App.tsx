import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Zap, ArrowRightLeft, ArrowDownUp, Wallet, ChevronRight, Sparkles } from 'lucide-react'

type Tab = 'zap-in' | 'zap-out'

export default function App() {
  const [activeTab, setActiveTab] = useState<Tab>('zap-in')
  const [isZapping, setIsZapping] = useState(false)

  return (
    <div className="min-h-screen bg-void text-mist font-sans selection:bg-gold selection:text-void">
      {/* Header */}
      <header className="flex items-center justify-between px-8 py-6 border-b border-stone/30">
        <div className="flex items-baseline gap-3">
          <Zap className="w-5 h-5 text-gold" strokeWidth={1.5} />
          <h1 className="text-lg font-light tracking-[0.2em] uppercase text-bone">
            Zap Soroswap
          </h1>
        </div>
        <button className="flex items-center gap-2 px-4 py-2 border border-stone/50 text-xs tracking-widest uppercase hover:border-gold hover:text-gold transition-colors duration-500">
          <Wallet className="w-3 h-3" strokeWidth={1.5} />
          Connect
        </button>
      </header>

      {/* Main */}
      <main className="max-w-6xl mx-auto px-8 py-16">
        <div className="mb-20">
          <p className="font-serif italic text-ash text-sm mb-4">One-click liquidity automation</p>
          <h2 className="font-serif text-5xl md:text-7xl text-bone leading-[0.9] max-w-2xl">
            Turn any token into a <span className="text-gold">position.</span>
          </h2>
        </div>

        {/* Tabs */}
        <div className="flex gap-8 mb-12 border-b border-stone/30 pb-4">
          <button
            onClick={() => setActiveTab('zap-in')}
            className={`flex items-center gap-2 text-sm tracking-widest uppercase transition-colors duration-300 ${
              activeTab === 'zap-in' ? 'text-gold' : 'text-ash hover:text-mist'
            }`}
          >
            <ArrowRightLeft className="w-4 h-4" strokeWidth={1.5} />
            Zap In
          </button>
          <button
            onClick={() => setActiveTab('zap-out')}
            className={`flex items-center gap-2 text-sm tracking-widest uppercase transition-colors duration-300 ${
              activeTab === 'zap-out' ? 'text-gold' : 'text-ash hover:text-mist'
            }`}
          >
            <ArrowDownUp className="w-4 h-4" strokeWidth={1.5} />
            Zap Out
          </button>
        </div>

        <AnimatePresence mode="wait">
          {isZapping ? (
            <motion.div
              key="zapping"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="flex items-center gap-4 text-ash"
            >
              <motion.div
                animate={{ rotate: 360 }}
                transition={{ repeat: Infinity, duration: 2, ease: "linear" }}
              >
                <Zap className="w-5 h-5 text-gold" strokeWidth={1.5} />
              </motion.div>
              <span className="text-sm tracking-widest uppercase">Processing</span>
            </motion.div>
          ) : (
            <motion.div
              key={activeTab}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              transition={{ duration: 0.3 }}
              className="grid md:grid-cols-2 gap-12"
            >
              <div className="space-y-8">
                <div>
                  <label className="block text-xs tracking-widest uppercase text-ash mb-3">Token</label>
                  <div className="flex items-center justify-between p-4 border border-stone/50 hover:border-ash transition-colors cursor-pointer">
                    <span className="text-bone">USDC</span>
                    <ChevronRight className="w-4 h-4 text-ash" strokeWidth={1.5} />
                  </div>
                </div>
                <div>
                  <label className="block text-xs tracking-widest uppercase text-ash mb-3">Amount</label>
                  <input type="number" placeholder="0.00" className="w-full bg-transparent border border-stone/50 p-4 text-bone placeholder:text-stone focus:border-gold focus:outline-none transition-colors" />
                </div>
                <div>
                  <label className="block text-xs tracking-widest uppercase text-ash mb-3">Pool</label>
                  <div className="p-4 border border-stone/50 hover:border-ash transition-colors cursor-pointer">
                    <div className="flex justify-between items-center">
                      <span className="text-bone">USDC / BRLT</span>
                      <span className="text-xs text-ash">APR 12.5%</span>
                    </div>
                  </div>
                </div>
                <button onClick={() => setIsZapping(true)} className="w-full py-4 bg-bone text-void text-sm tracking-widest uppercase hover:bg-gold transition-colors duration-300">
                  Preview Zap
                </button>
              </div>

              <div className="border-l border-stone/30 pl-12">
                <div className="mb-8">
                  <p className="font-serif italic text-ash text-sm mb-2">Estimate</p>
                  <h3 className="text-2xl text-bone font-light">Preview your position</h3>
                </div>
                <div className="space-y-6">
                  <div className="flex justify-between items-baseline border-b border-stone/20 pb-4">
                    <span className="text-xs tracking-widest uppercase text-ash">Split</span>
                    <span className="text-bone">50% / 50%</span>
                  </div>
                  <div className="flex justify-between items-baseline border-b border-stone/20 pb-4">
                    <span className="text-xs tracking-widest uppercase text-ash">LP Tokens</span>
                    <span className="text-bone font-serif text-xl">~4.5</span>
                  </div>
                  <div className="flex justify-between items-baseline border-b border-stone/20 pb-4">
                    <span className="text-xs tracking-widest uppercase text-ash">Slippage</span>
                    <span className="text-gold">0.5%</span>
                  </div>
                  <div className="flex justify-between items-baseline">
                    <span className="text-xs tracking-widest uppercase text-ash">Network Fee</span>
                    <span className="text-ash">~0.001 XLM</span>
                  </div>
                </div>
                <div className="mt-12 pt-8 border-t border-stone/30">
                  <div className="flex items-center gap-2 text-ash text-xs">
                    <Sparkles className="w-3 h-3" strokeWidth={1.5} />
                    <span>Atomic execution — all or nothing</span>
                  </div>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </main>

      <footer className="px-8 py-6 border-t border-stone/30 mt-20">
        <div className="flex justify-between items-center text-xs text-ash tracking-widest uppercase">
          <span>Stellar Testnet</span>
          <span>Soroban SDK v26.0.0</span>
        </div>
      </footer>
    </div>
  )
}

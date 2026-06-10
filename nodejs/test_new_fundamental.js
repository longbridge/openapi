const { FundamentalContext, Config, OAuth } = require('./index.js')

const pf = { pass: 0, fail: 0 }
async function check(name, fn) {
  process.stdout.write(`  ${name.padEnd(58)}`)
  try {
    const r = await fn()
    console.log(`OK  ${r}`)
    pf.pass++
  } catch (e) {
    console.log(`FAIL  ${e.message || e}`)
    pf.fail++
  }
}

async function main() {
  const oauth = await OAuth.build('fd52fbc5-02a9-47f5-ad30-0842c841aae9', (_, url) => {
    console.log('Open:', url)
  })
  const cfg = Config.fromOAuth(oauth)
  const ctx = FundamentalContext.new(cfg)

  console.log('\n=== FundamentalContext — new APIs (Node.js) ===')

  await check('businessSegments(AAPL.US)', async () => {
    const r = await ctx.businessSegments('AAPL.US')
    return `${r.business.length} segments`
  })

  await check('businessSegmentsHistory(AAPL.US, qf, null)', async () => {
    const r = await ctx.businessSegmentsHistory('AAPL.US', 'qf', null)
    return `${r.historical.length} periods`
  })

  await check('institutionRatingViews(AAPL.US)', async () => {
    const r = await ctx.institutionRatingViews('AAPL.US')
    return `${r.elist.length} months`
  })

  let bkId = 'BK/US/IN00258'
  await check('industryRank(US, 0, 0, 10)', async () => {
    const r = await ctx.industryRank('US', '0', '0', 10)
    const total = r.items.reduce((s, g) => s + g.lists.length, 0)
    if (r.items[0]?.lists[0]?.counterId) bkId = r.items[0].lists[0].counterId
    return `${total} industries, first: ${bkId}`
  })

  await check('industryPeers(BK/US/..., US, null)', async () => {
    const r = await ctx.industryPeers(bkId, 'US', null)
    const children = r.chain?.next?.length ?? 0
    return `top:${r.top.name} children:${children}`
  })

  await check('financialReportSnapshot(AAPL.US)', async () => {
    const r = await ctx.financialReportSnapshot('AAPL.US', null, null, null)
    return `${r.ticker} ${r.fpStart}–${r.fpEnd}`
  })

  console.log(`\nPASS: ${pf.pass}  FAIL: ${pf.fail}`)
  if (pf.fail > 0) process.exit(1)
}

main().catch(e => { console.error(e); process.exit(1) })

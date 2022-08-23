import { useI18n } from '@solid-primitives/i18n'


const createApp = () => {
  const [_, { locale }] = useI18n()

  const nextLanguage = () => {
    const next = locale() === 'en' ? 'fr' : 'en'
    locale(next)
  }

  const theme = () => {
  }

  return {
    nextLanguage,
    theme,
  }
}

export {
  createApp,
}
